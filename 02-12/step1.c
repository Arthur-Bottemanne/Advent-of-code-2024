#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LINE_LENGTH 256
#define MAX_ROWS 1000
#define MAX_COLUMNS 8

void removeElement(int arr[], int size, int index) {
    if (index < 0 || index >= size) {
        printf("Invalid index\n");
        return;
    }

    for (int i = index; i < size - 1; i++) {
        arr[i] = arr[i + 1];
    }

    arr[size - 1] = 0;

    (size)--;
}

int checkIfSafeWithoutLevel(int inputIntegersLine[]) {
    int isUnsafe = 0;

    for (int i = 0; i < MAX_COLUMNS; i++)
    {
        int newInputIntegersLine[MAX_COLUMNS] = {0};

        for (int j = 0; j < MAX_COLUMNS; j++) {
            newInputIntegersLine[j] = inputIntegersLine[j];
        }

        removeElement(newInputIntegersLine, MAX_COLUMNS, i);

        int isIncreasing = 0;
        int previousNumber = 0;

        for (int j = 1; j < MAX_COLUMNS; j++)
        {
            int currentNumber = newInputIntegersLine[j];
            int distance = abs(previousNumber - currentNumber);

            if (currentNumber == 0)
            {
                break;
            }

            if (j == 1)
            {
                previousNumber = newInputIntegersLine[0];
                distance = abs(currentNumber - previousNumber);

                if (previousNumber - currentNumber < 0)
                {
                    isIncreasing = 1;
                }
            }

            if (distance < 4 && distance > 0)
            {
                if ((isIncreasing && previousNumber - currentNumber > 0) || (!isIncreasing && previousNumber - currentNumber < 0))
                {
                    isUnsafe++;
                    break; // unsafe
                }
            }
            else
            {
                isUnsafe++;
                break; // unsafe
            }

            previousNumber = currentNumber;
        }
    }

    if (isUnsafe < 8)
    {
        isUnsafe = 0;
    }
    else
    {
        isUnsafe = 1;
    }

    return isUnsafe;
}

int main() {
    FILE *input;

    input = fopen("input", "r");

    if (input == NULL) {
        perror("Failed to open file");
        return 1;
    }

    int inputIntegers[MAX_ROWS][MAX_COLUMNS] = {0};
    int row = 0;

    char line[MAX_LINE_LENGTH];
    while (fgets(line, sizeof(line), input) && row < MAX_ROWS) {
        int column = 0;
        char *token = strtok(line, " ");
        while (token != NULL && column < MAX_COLUMNS) {
            inputIntegers[row][column] = atoi(token);
            column++;
            token = strtok(NULL, " ");
        }
        row++;
    }

    fclose(input);

    int previousNumber = 0;
    int amountUnsafe = 0;

    for (int i = 0; i < row; i++)
    {
        int isIncreasing = 0;

        for (int j = 1; j < MAX_COLUMNS; j++)
        {
            int currentNumber = inputIntegers[i][j];
            int distance = abs(previousNumber - currentNumber);

            if (currentNumber == 0)
            {
                break;
            }

            if (j == 1)
            {
                previousNumber = inputIntegers[i][0];
                distance = abs(currentNumber - previousNumber);

                if (previousNumber - currentNumber < 0)
                {
                    isIncreasing = 1;
                }
            }

            if (distance < 4 && distance > 0)
            {
                if ((isIncreasing && previousNumber - currentNumber > 0) || (!isIncreasing && previousNumber - currentNumber < 0))
                {
                    int isUnsafe = checkIfSafeWithoutLevel(inputIntegers[i]);

                    if (isUnsafe)
                    {
                        amountUnsafe++;
                        break; // unsafe
                    }

                    break; // safe
                }
            }
            else
            {
                int isUnsafe = checkIfSafeWithoutLevel(inputIntegers[i]);

                if (isUnsafe)
                {
                    amountUnsafe++;
                    break; // unsafe
                }

                break; //safe
            }

            previousNumber = currentNumber;
        }
    }

    printf("Amount safe: %d", (row - amountUnsafe));

    return 0;
}
