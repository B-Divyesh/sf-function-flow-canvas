#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

const SERVER: &str = r#"#!/usr/bin/env python3
import json, sys
def send(value):
    body=json.dumps(value,separators=(',',':')).encode()
    sys.stdout.buffer.write(('Content-Length: %d\r\n\r\n'%len(body)).encode()+body);sys.stdout.buffer.flush()
while True:
    length=None
    while True:
        line=sys.stdin.buffer.readline()
        if not line: sys.exit(0)
        if line in (b'\r\n',b'\n'): break
        if line.lower().startswith(b'content-length:'): length=int(line.split(b':',1)[1])
    msg=json.loads(sys.stdin.buffer.read(length))
    if 'id' not in msg: continue
    method=msg.get('method'); ident=msg['id']
    if method=='initialize': result={'capabilities':{'callHierarchyProvider':True,'hoverProvider':True}}
    elif method=='textDocument/prepareCallHierarchy':
        uri=msg['params']['textDocument']['uri']; result=[{'name':'root','kind':12,'uri':uri,'range':{'start':{'line':0,'character':3},'end':{'line':0,'character':7}},'selectionRange':{'start':{'line':0,'character':3},'end':{'line':0,'character':7}}}]
    elif method=='textDocument/hover': result={'contents':{'kind':'plaintext','value':'fn()'}}
    elif method=='callHierarchy/incomingCalls': result=[]
    elif method=='callHierarchy/outgoingCalls':
        item=msg['params']['item']; result=[] if item['name']=='child' else [{'to':{'name':'child','detail':'fn child()','kind':12,'uri':item['uri'],'range':{'start':{'line':1,'character':3},'end':{'line':1,'character':8}},'selectionRange':{'start':{'line':1,'character':3},'end':{'line':1,'character':8}}},'fromRanges':[]}]
    else: result=None
    send({'jsonrpc':'2.0','id':ident,'result':result})
"#;

const CYCLIC_SERVER: &str = r#"#!/usr/bin/env python3
import json, sys
def send(value):
    body=json.dumps(value,separators=(',',':')).encode()
    sys.stdout.buffer.write(('Content-Length: %d\r\n\r\n'%len(body)).encode()+body);sys.stdout.buffer.flush()
def item(name, uri, line):
    return {'name':name,'detail':'fn '+name+'()','kind':12,'uri':uri,'range':{'start':{'line':line,'character':3},'end':{'line':line,'character':7}},'selectionRange':{'start':{'line':line,'character':3},'end':{'line':line,'character':7}}}
while True:
    length=None
    while True:
        line=sys.stdin.buffer.readline()
        if not line: sys.exit(0)
        if line in (b'\r\n',b'\n'): break
        if line.lower().startswith(b'content-length:'): length=int(line.split(b':',1)[1])
    msg=json.loads(sys.stdin.buffer.read(length))
    if 'id' not in msg: continue
    method=msg.get('method'); ident=msg['id']
    if method=='initialize': result={'capabilities':{'callHierarchyProvider':True,'hoverProvider':True}}
    elif method=='textDocument/prepareCallHierarchy': result=[item('root',msg['params']['textDocument']['uri'],0)]
    elif method=='textDocument/hover': result={'contents':{'kind':'plaintext','value':'fn()'}}
    elif method in ('callHierarchy/incomingCalls','callHierarchy/outgoingCalls'):
        current=msg['params']['item']; result=[] if current['name']=='peer' else ([{'from':item('peer',current['uri'],1),'fromRanges':[]}] if method.endswith('incomingCalls') else [{'to':item('peer',current['uri'],1),'fromRanges':[]}])
    else: result=None
    send({'jsonrpc':'2.0','id':ident,'result':result})
"#;

const COLD_SERVER: &str = r#"#!/usr/bin/env python3
import json, sys, time
prepare_count=0
def send(value):
    body=json.dumps(value,separators=(',',':')).encode()
    sys.stdout.buffer.write(('Content-Length: %d\r\n\r\n'%len(body)).encode()+body);sys.stdout.buffer.flush()
while True:
    length=None
    while True:
        line=sys.stdin.buffer.readline()
        if not line: sys.exit(0)
        if line in (b'\r\n',b'\n'): break
        if line.lower().startswith(b'content-length:'): length=int(line.split(b':',1)[1])
    msg=json.loads(sys.stdin.buffer.read(length))
    if 'id' not in msg: continue
    method=msg.get('method'); ident=msg['id']
    if method=='initialize': result={'capabilities':{'callHierarchyProvider':True,'hoverProvider':True}}
    elif method=='textDocument/prepareCallHierarchy':
        prepare_count+=1
        if prepare_count==1:
            time.sleep(6.25)
            send({'jsonrpc':'2.0','id':ident,'error':{'code':-32801,'message':'content modified'}}); continue
        else:
            uri=msg['params']['textDocument']['uri']; result=[{'name':'root','kind':12,'uri':uri,'range':{'start':{'line':0,'character':3},'end':{'line':0,'character':7}},'selectionRange':{'start':{'line':0,'character':3},'end':{'line':0,'character':7}}}]
    elif method=='textDocument/hover': result={'contents':{'kind':'plaintext','value':'fn()'}}
    elif method in ('callHierarchy/incomingCalls','callHierarchy/outgoingCalls'): result=[]
    else: result=None
    send({'jsonrpc':'2.0','id':ident,'result':result})
"#;

#[test]
fn documented_html_and_json_flows_work_end_to_end() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("service.custom");
    let server = temp.path().join("mock-lsp.py");
    let html = temp.path().join("flow.html");
    fs::write(&source, "fn root() { child(); }\nfn child() {}\n").unwrap();
    fs::write(&server, SERVER).unwrap();
    fs::set_permissions(&server, fs::Permissions::from_mode(0o755)).unwrap();
    let binary = env!("CARGO_BIN_EXE_ffc");

    let output = Command::new(binary)
        .arg(&source)
        .args(["--symbol", "root", "--server"])
        .arg(&server)
        .args(["--root"])
        .arg(temp.path())
        .args(["--out"])
        .arg(&html)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let page = fs::read_to_string(&html).unwrap();
    assert!(page.contains("<dd>2</dd>"));
    assert!(page.contains("child"));

    let output = Command::new(binary)
        .arg(&source)
        .args(["--symbol", "root", "--server"])
        .arg(&server)
        .args(["--root"])
        .arg(temp.path())
        .arg("--json")
        .output()
        .unwrap();
    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["nodes"].as_array().unwrap().len(), 2);
    assert_eq!(value["edges"].as_array().unwrap().len(), 1);
}

#[test]
fn mutual_calls_are_kept_in_both_canvas_lanes() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("service.custom");
    let server = temp.path().join("cyclic-lsp.py");
    fs::write(&source, "fn root() { peer(); }\nfn peer() { root(); }\n").unwrap();
    fs::write(&server, CYCLIC_SERVER).unwrap();
    fs::set_permissions(&server, fs::Permissions::from_mode(0o755)).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_ffc"))
        .arg(&source)
        .args(["--symbol", "root", "--server"])
        .arg(&server)
        .args(["--root"])
        .arg(temp.path())
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let flow: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let nodes = flow["nodes"].as_array().unwrap();
    assert!(nodes
        .iter()
        .any(|node| node["name"] == "peer" && node["side"] == "inbound"));
    assert!(nodes
        .iter()
        .any(|node| node["name"] == "peer" && node["side"] == "outbound"));
    assert_eq!(flow["edges"].as_array().unwrap().len(), 2);
}

#[test]
fn cold_prepare_responses_recover_without_a_second_invocation() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("service.custom");
    let server = temp.path().join("cold-lsp.py");
    fs::write(&source, "fn root() {}\n").unwrap();
    fs::write(&server, COLD_SERVER).unwrap();
    fs::set_permissions(&server, fs::Permissions::from_mode(0o755)).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_ffc"))
        .arg(&source)
        .args(["--symbol", "root", "--server"])
        .arg(&server)
        .args(["--root"])
        .arg(temp.path())
        .args(["--timeout", "10"])
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let flow: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(flow["nodes"].as_array().unwrap().len(), 1);
}
