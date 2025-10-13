def main():
    message = "Hello, World!"
    print(message)
    
    numbers = [1, 2, 3, 4, 5]
    for num in numbers:
        if num % 2 == 0:
            print(f"Even: {num}")
        else:
            print(f"Odd: {num}")

if __name__ == "__main__":
    main()
