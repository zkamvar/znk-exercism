from pathlib import Path
import tomlkit as tommy
import sys
import subprocess

def get() -> int:
    print("Hello from expy!")
    # read in the text of the pyproject file
    dic = tommy.loads(Path("pyproject.toml").read_text())
    commands = sys.argv[1:]
    exercise = commands[-1].split("=")[-1]

    # add the exercise folder to the workspace
    dic["tool"]["rye"]["workspace"]["members"].append(exercise)

    # run the exercism command
    completed = subprocess.run(commands, check = True)
    # if it's successful, init the project
    subprocess.run(["rye", "init"], cwd = exercise, check = True)

    # write the pyproject file
    with Path("pyproject.toml").open("w") as fout:
        fout.write(tommy.dumps(dic))
    fout.close()

    # sync the project
    subprocess.run(["rye", "sync"])
    return 0
