[![rust-clippy analyze](https://github.com/LMA1980/api.jelomu.local/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/LMA1980/api.jelomu.local/actions/workflows/rust-clippy.yml)
# api.jelomu.local - a study using Rust Rocket Framework

## _Disclaimer:_
> Provided along is a tool for helping to configure a study environment where 
> security is as secure you can make it and _definitely not for a public usage_ though.
> 
> The solution is implemented against openssl. Provided along, are configuration
> examples that support the latest algorithm supported by modern web browser.
> 
>  - ca.cert.conf: I don't want to require to replace this certificate too often, and the 
>    public certification portion requires to be deployed to your browser's trusted 
>    authorities. I manually replace it on needed basis though its create a 30 years 
>    certificate.
>  
>           Depending on the system or software, the certificate may need to be
>           deployed to the host, or to within the software itself. This portion
>           is well documented.
>
>  - server.cert.conf: provide the configuration for the server. This one we will want to
>    replace within 90 days.
> 
> It tries to re-enforces the new standard of 90 days expirations. This may force me
> to provide solution to handles this before the certificate expires, or before trying
> to execute 'cargo run' the project.
> 
> Though. Again, this is not a production solution. Research Let's Encrypt solution for
> anything more serious.

## Goal:
First, _serious_ implementation using Rust and Rocket: I am a beginner at Rust and Rocky 
appeared to provide the most strait forward solution.

Milestone 1:
    Provide a public interface.
    [x] About (json)
    [ ] Online Documentation (Html)
    [ ] Authenticate
    [ ] Authorisation