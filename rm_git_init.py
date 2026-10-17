import os
import shutil
import stat

def remove_readonly(func, path, excinfo):
    """Clear the readonly bit and re-attempt file deletion (Fix for Windows Git files)"""
    os.chmod(path, stat.S_IWRITE)
    func(path)

def rm_git_init():
    base_path = os.path.expanduser("~/Programming/rust-lang")
    exclude_dirs = {'temp', 'cache', '.git'}  # Set of directory names to ignore

    target_dir = [
        d for d in os.listdir(base_path)
        if os.path.isdir(os.path.join(base_path, d)) and d not in exclude_dirs
    ]

    delete_count = 0
    # print(dir)
    # loop through and delete internal ".git" 
    for folder in target_dir:
        git_dir_path = os.path.join(base_path, folder, ".git")

        # check if the `.git` actually exists
        if os.path.exists(git_dir_path) and os.path.isdir(git_dir_path):
            print(f"Deleting: {git_dir_path}")
            shutil.rmtree(git_dir_path)
            delete_count += 1

    if delete_count == 0:
        print("No active .git folders found to delete.")
    else:
        print(f"Successfully cleaned up {delete_count} .git repository folder(s).")

def main():
    choice = input("Do you want to perform this operation(y/N): ").strip().lower()
    if (choice == 'y'):
        rm_git_init()
    else:
        print("Exiting this program. No changes made.")

if __name__ == '__main__':
    main()
