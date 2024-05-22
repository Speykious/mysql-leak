package com.leak;

import java.sql.*;
import java.util.Random;

public class Main {
    public static void main(String[] argv) {
        String databaseURL = System.getenv("DATABASE_URL");
        if (databaseURL == null) {
            System.err.println("Error: Please set the DATABASE_URL env variable");
            return;
        }

        try {
            Class.forName("com.mysql.cj.jdbc.Driver");
        } catch (Exception e) {
            System.err.println("Error when registering driver");
            e.printStackTrace();
        }

        int bulkCount = 1000;

        Random rand = new Random();

        try (Connection conn = DriverManager.getConnection("jdbc:" + databaseURL)) {
            StringBuilder queryBuilder = new StringBuilder(
                    "INSERT INTO mysqlleak_demo.values (rand_a, rand_b, rand_s) VALUES");
            for (int b = 0; b < bulkCount; b++) {
                if (b > 0)
                    queryBuilder.append(',');

                queryBuilder.append("\n    (?, ?, ?)");
            }

            PreparedStatement stmt = conn.prepareStatement(queryBuilder.toString());

            System.out.println("testing bulk inserts: start");

            for (int i = 0; i < 10_000; i++) {
                System.out.println("  bulk insert #" + (i + 1) + " (count = " + bulkCount + ")");

                int paramIdx = 0;
                for (int b = 0; b < bulkCount; b++) {
                    stmt.setInt(++paramIdx, randomNumber(rand));
                    stmt.setInt(++paramIdx, randomNumber(rand));
                    stmt.setString(++paramIdx, randomString(rand));
                }

                stmt.execute();
            }

            System.out.println("testing bulk inserts: stop");
        } catch (Exception e) {
            System.err.println("Error when bulk-inserting to the table");
            e.printStackTrace();
        }
    }

    private static int randomNumber(Random rand) {
        return rand.nextInt(2_000_001) - 1_000_000;
    }

    private static String randomString(Random rand) {
        int length = rand.nextInt(100) + 1;
        byte[] buffer = new byte[length];
        for (int i = 0; i < length; i++) {
            buffer[i] = (byte) ((byte) 'a' + (byte) rand.nextInt((int) 'z' - (int) 'a' + 1));
        }
        return new String(buffer);
    }
}