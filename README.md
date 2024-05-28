# teachable02
Solana program in "Rust + Solana For Beginners" course at careerbooster.teachable.com.
create PDA account using invoke_signed to virtually sign an invocation on behalf of program-derived address.

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```

### Deploy BPF on localhost and test by typescript client
```
(1 session to start local solana validator)
$ solana-test-validator
(1 session to monitor logs)
$ solana logs
(1 session to deploy on-chain program and execute typescript client)
$ solana program deploy target/deploy/teachable02.so
Program Id: AvJadcJAWgZVbobSCerqLz3JmWLget9GpfdA8cDFTfFv
(you need to update your programId at line 12 in typesriptclient)
$ npx esrun create-pda.ts
```

