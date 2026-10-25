import os

file_name = ".gitignore"
file_contents = "/target"

def create_gitignore():
    base_path = os.path.expanduser("~/Programming/rust-lang")
    exclude_dirs = {'temp', 'cache', '.git'} 
    target_dir = [
        d for d in os.listdir(base_path)
        if os.path.isdir(os.path.join(base_path, d)) and d not in exclude_dirs   
    ]

    print(target_dir)

    for item in target_dir:
        gitignore_path = os.path.join(base_path, item, file_name)

        if not os.path.isfile(gitignore_path):
            with open(gitignore_path, "w") as f:
                f.write(file_contents)
            print(f"Created: {gitignore_path}")
        else:
            print(f"Skipped: {gitignore_path} already exists")

    print(f"[PID: {os.getpid()}] Process Completed.")
        

if __name__ == '__main__':
    choice = input("Do you want to perform this operation(y/N): ").strip().lower()
    if (choice == 'y'):
        create_gitignore()
    else:
        print("Exiting this program. No changes made.")