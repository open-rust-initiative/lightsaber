## Lightsaber proposal

#### Design Target

This project is a Rust library designed to enable developers to define a series of functionalities. These functionalities can then be used by downstream users to define their own code logic and develop in a manner similar to using a programming language, thereby facilitating a more intuitive and flexible development process.

By integrating this library, developers can define specific functions, structures, and interfaces, along with a simple syntax for their custom language. The end result is a simple language interpreter that can be provided to other users. The unique syntax and behavior of this custom language are defined by the developers themselves, using the Rust language.


> Developers using Lightsaber:
> 1. Import this rust library
> 2. Define functions (abilities)
> 3. Define syntax
> 4. Build and release his program
> 
> Users:
> 1. Download the precompiled program and use it as an "language interpreter"
> 2. **Write his own logic (in a simpler way) to achieve specific goals**


#### Ideas

What we need is a language that falls between a full-featured programming language(e.g. Python, Javascript...) and pure data definition(e.g. JSON, Yaml...). We implement the framework for this language in Rust, and allow developers who use our project as a library to extend the framework of this language with functions defined in Rust.

HCL(https://github.com/hashicorp/hcl) closely aligns with our vision:

>Newcomers to HCL often ask: why not JSON, YAML, etc?
>
>Whereas JSON and YAML are formats for serializing data structures, HCL is a syntax and API specifically designed for building structured configuration formats.
>
>HCL attempts to strike a compromise between generic serialization formats such as JSON and configuration formats built around full programming languages such as Ruby. HCL syntax is designed to be easily read and written by humans, and allows *declarative* logic to permit its use in more complex applications.

#### Inspiration & Reference

##### Nginx

- Configuration file with custom syntax
  Supports **Directives**, **Blocks**, **Contexts**, **Variables**

  ```nginx
  http {
      # Define a variable
      set $backend "localhost:8080";
  
      server {
          listen 80;
          server_name localhost;
  
          location / {
              # Use the variable in the proxy_pass directive
              proxy_pass http://$backend;
          }
  
          location ~* \.(jpg|jpeg|png|gif|ico|css|js)$ {
              expires 30d;
              access_log off;
              try_files $uri @backend;
          }
  
          location @backend {
              proxy_pass http://$backend;
          }
      }
  }
  ```

- Support Lua Module

##### Terraform*

- Has its own configuration language (HCL)

- https://developer.hashicorp.com/terraform/language

- **Resource blocks**, **Variables**, **Outputs**

- Also Functions, Math calculation, Conditions

    ```hcl
    terraform {
      required_providers {
        cloudflare = {
          source  = "cloudflare/cloudflare"
          version = "~> 4"
        }
      }
    }
    
    variable "CLOUDFLARE_ACCOUNT_ID" {
      # read account id from $TF_VAR_CLOUDFLARE_ACCOUNT_ID
      type = string
    }
    
    resource "cloudflare_workers_kv_namespace" "uptimeflare_kv" {
      account_id = var.CLOUDFLARE_ACCOUNT_ID
      title      = "uptimeflare_kv"
    }
    
    resource "cloudflare_worker_script" "uptimeflare" {
      account_id         = var.CLOUDFLARE_ACCOUNT_ID
      name               = "uptimeflare_worker"
      content            = file("worker/dist/index.js")
      module             = true
      compatibility_date = "2023-11-08"
    
      kv_namespace_binding {
        name         = "UPTIMEFLARE_STATE"
        namespace_id = cloudflare_workers_kv_namespace.uptimeflare_kv.id
      }
    }
    
    resource "cloudflare_pages_project" "uptimeflare" {
      account_id        = var.CLOUDFLARE_ACCOUNT_ID
      name              = "uptimeflare"
      production_branch = "main"
    
      deployment_configs {
        production {
          kv_namespaces = {
            UPTIMEFLARE_STATE = cloudflare_workers_kv_namespace.uptimeflare_kv.id
          }
          compatibility_date  = "2023-11-08"
          compatibility_flags = ["nodejs_compat"]
        }
      }
    }
    ```

##### Frida

- https://github.com/orgs/frida/repositories?type=all

- Create (or Auto-generate) bindings for different languages (Python, Javascript, Swift...)

- ...Or embed lightweight Javascript engines, e.g. https://bellard.org/quickjs/
  > QuickJS is a small and embeddable Javascript engine. It supports the [ES2023](https://tc39.github.io/ecma262/2023) specification including modules, asynchronous generators, proxies and BigInt.
  > It optionally supports mathematical extensions such as big decimal floating point numbers (BigDecimal), big binary floating point numbers (BigFloat) and **operator overloading**.
  >
  > - Can compile Javascript sources to executables with no external dependency.
  >- Small built-in standard library with C library wrappers.