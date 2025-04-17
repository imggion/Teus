#!/usr/bin/env python3
import uuid
# import tomli_w
from pathlib import Path

def main():
    # Generate UUID4 as secret salt
    secret = str(uuid.uuid4())
    
    # Define output file path
    output_file = "secret.toml"
    
    # Create directory if it doesn't exist
    Path(output_file).parent.mkdir(parents=True, exist_ok=True)
    
    # Write secret to TOML file
    # with open(output_file, "wb") as f:
    #     tomli_w.dump({"secret_salt": secret}, f)
    print(f"Secret Generated -> {secret}")
    print(f"Secret UUID4 generated and written to {output_file}")

if __name__ == "__main__":
    main()
