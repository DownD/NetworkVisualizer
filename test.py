import pandas as pd



def main():
    # Open a test.txt file
    with open('test.txt', 'r') as f:
        # Read the file line by line
        for line in f:
            #warder(line)
            print(line)

    

if __name__ == '__main__':
    main()
