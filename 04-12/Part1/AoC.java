import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

/**
 * AoC
 */
public class AoC {
    static final int TOTAL_ROWS = 142;
    static final int TOTAL_COLUMNS = 142;
    static final int[][] DIRECTIONS = {{1, 0},{1, 1},{0, 1},{-1, 1}, {-1, 0}, {-1, -1}, {0, -1}, {1, -1}};
    private static char[][] inputCharacters = new char[TOTAL_ROWS][TOTAL_COLUMNS];

    public static void main(String[] args) {
        System.out.println("AoC 2024 day 4:");

        try {
            File input = new File("./04-12/Part1/input");
            Scanner scanner = new Scanner(input);

            String data = "";

            int currentRow = 0;

            while (scanner.hasNextLine()) {
                if (currentRow != 0)
                {
                    data = scanner.nextLine();
                }
                
                for (int i = 0; i < TOTAL_COLUMNS; i++)
                {
                    if (currentRow == 0 || currentRow == (TOTAL_ROWS - 1) || i == 0 || i == (TOTAL_COLUMNS - 1))
                    {
                        inputCharacters[currentRow][i] = '0';
                    }
                    else
                    {
                        inputCharacters[currentRow][i] = data.charAt(i - 1);
                    }
                }

                currentRow++;
            }

                            
            for (int i = 0; i < TOTAL_COLUMNS; i++)
            {
                if (currentRow == 0 || currentRow == (TOTAL_ROWS - 1) || i == 0 || i == (TOTAL_COLUMNS - 1))
                {
                    inputCharacters[currentRow][i] = '0';
                }
            }

            for (int row = 0; row < inputCharacters.length; row++) {
                for (int col = 0; col < inputCharacters[row].length; col++) {
                    System.out.print(inputCharacters[row][col]);
                }
                System.out.println();
            }

            int result = 0;

            for (int i = 0; i < inputCharacters.length; i++)
            {
                for (int j = 0; j < inputCharacters[i].length; j++)
                {
                    if (inputCharacters[i][j] == 'X')
                    {
                        result += findXmas(i, j);
                    }
                }
            }

            System.out.println("Result: " + result);
            scanner.close();
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
    }

    private static int findXmas(int row, int column) {
        System.out.println("X at: " + row + ", " + column);

        int XmasFound = 0;

        for (int[] direction : DIRECTIONS) {
            int rowDirection = direction[0];
            int columnDirection = direction[1];

            if (inputCharacters[row + rowDirection][column + columnDirection] == 'M')
            {
                if (inputCharacters[row + rowDirection * 2][column + columnDirection * 2] == 'A')
                {
                    if (inputCharacters[row + rowDirection * 3][column + columnDirection * 3] == 'S')
                    {
                        XmasFound++;
                    }
                }
            }
        }

        return XmasFound;
    }
}