package org.example;

import java.util.*;

import static org.example.Day23.*;

public class Day23 {

    public static final String FINAL_STATE_ENCODED = "...........ABCDABCD";
    public static final HashMap<Character, Integer> FINAL_COST_MAP = new HashMap<Character, Integer>() {{
        put('A', 1);
        put('B', 10);
        put('C', 100);
        put('D', 1000);
    }};

    public static void main(String[] args) {
//        String initialState = "...........BCBDADCA";
        String initialState = "...........DAADCCBB";
        org.example.Day23 solution = new org.example.Day23();
        State initial = new State();
        initial.cost = 0;
        initial.position = initialState;
        System.out.println(solution.shortest_path_dijkstra(initial));
        System.out.println(solution.shortest_path_memoization(initial));
    }

    public static char[][] decode(String s) {
        char[][] board = new char[3][11];
        for (int i = 0; i < 11; i++) {
            board[2][i] = s.charAt(i);
        }
        Arrays.fill(board[0], '#');
        Arrays.fill(board[1], '#');
        int idx = 11;
        board[1][2] = s.charAt(idx++);
        board[1][4] = s.charAt(idx++);
        board[1][6] = s.charAt(idx++);
        board[1][8] = s.charAt(idx++);
        board[0][2] = s.charAt(idx++);
        board[0][4] = s.charAt(idx++);
        board[0][6] = s.charAt(idx++);
        board[0][8] = s.charAt(idx++);
        return board;
    }

    public static String encode(char[][] board) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < 11; i++) {
            sb.append(board[2][i]);
        }
        sb.append(board[1][2]);
        sb.append(board[1][4]);
        sb.append(board[1][6]);
        sb.append(board[1][8]);
        sb.append(board[0][2]);
        sb.append(board[0][4]);
        sb.append(board[0][6]);
        sb.append(board[0][8]);
        return sb.toString();
    }

    public long shortest_path_dijkstra(State initial) {
        Map<String, Long> distance = new HashMap<>();
        PriorityQueue<State> pq = new PriorityQueue<>(Comparator.comparing(State::getCost));
        distance.put(initial.position, initial.cost);
        pq.offer(initial);
        while (!pq.isEmpty()) {
            State u = pq.poll();
            if (FINAL_STATE_ENCODED.equals(u.position)) {
                return u.cost;
            }
            if (u.cost > distance.getOrDefault(u.position, Long.MAX_VALUE)) {
                continue;
            }
            for (State nextState: u.neighbours()) {
                if (nextState.cost < distance.getOrDefault(nextState.position, Long.MAX_VALUE)) {
                    distance.put(nextState.position, nextState.cost);
                    pq.offer(nextState);
                }
            }
        }
        return -1;
    }

    public long shortest_path_memoization(State initial) {
        Map<String, Long> cache = new HashMap<>();
        long ret = shortest_path_2(initial, cache);
        return ret;
    }

    public long shortest_path_2(State state, Map<String, Long> cache) {
        if (FINAL_STATE_ENCODED.equals(state.position)) {
            return 0;
        }
        if (cache.containsKey(state.position)) {
            return cache.get(state.position);
        }
        long cost = Long.MAX_VALUE / 2; // 代表不可达
        for (State nextState : state.neighbours()) {
            long d = shortest_path_2(nextState, cache);
            if (cost > d + Math.abs(nextState.cost - state.cost)) {
                cost = d + Math.abs(nextState.cost - state.cost);
            }
        }
        cache.put(state.position, cost);
        return cost;
    }

}

class State {
    long cost;
    String position;

    public long getCost() {
        return cost;
    }

    @Override
    public String toString() {
        return "State{" +
                "cost=" + cost +
                ", position='" + position + '\'' +
                '}';
    }

    public List<State> neighbours() {
        char[][] board = decode(position);
        List<State> ret = new ArrayList<>();
        // 第一部分，hallway -> room
        for (int i = 0; i < 11; i++) {
            char curr = board[2][i];
            if (curr != '.') {
                int dstCol = (curr - 'A') * 2 + 2;
                // hallway to dstCol clear
                boolean hallwayClear = hallwayClearHelper(board, i, dstCol);
                // dstCol both empty or board[0][dstCol] == curr && board[1][dstCol] == '.'
                if (hallwayClear && ((board[0][dstCol] == curr && board[1][dstCol] == '.') || (board[0][dstCol] == '.' && board[1][dstCol] == '.'))) {
                    int dstRow = board[0][dstCol] == '.' && board[1][dstCol] == '.' ? 0 : 1;
                    board[dstRow][dstCol] = curr;
                    board[2][i] = '.';
                    State nextState = new State();
                    nextState.position = encode(board);
                    nextState.cost = cost + (long) (Math.abs(2 - dstRow) + Math.abs(i - dstCol)) * FINAL_COST_MAP.get(curr);
                    board[dstRow][dstCol] = '.';
                    board[2][i] = curr;
                    ret.add(nextState);
                }
            }
        }
        // 第二部分，room -> hallway
        for (int i = 0; i < 2; i++) {
            for (int j = 2; j < 10; j += 2) {
                char curr = board[i][j];
                if (curr != '.') {
                    // 如果是放好的，不动
                    int finalCol = (curr - 'A') * 2 + 2;
                    if (finalCol == j && (i == 0 || board[1 - i][j] == curr)) {
                        continue;
                    }
                    int dstRow = 2;
                    for (int dstCol = 0; dstCol < 11; dstCol += 1) {
                        // 上方不能放
                        if (dstCol == 2 || dstCol == 4 || dstCol == 6 || dstCol == 8) {
                            continue;
                        }
                        // 终点被占了也不行
                        if (board[dstRow][dstCol] != '.') {
                            continue;
                        }
                        boolean colClear = i == 1 || board[i + 1][j] == '.';
                        // hallway to dstCol clear
                        boolean hallwayClear = hallwayClearHelper(board, j, dstCol);
                        if (colClear && hallwayClear) {
                            board[dstRow][dstCol] = curr;
                            board[i][j] = '.';
                            State nextState = new State();
                            nextState.position = encode(board);
                            nextState.cost = cost + (long) (Math.abs(i - dstRow) + Math.abs(j - dstCol)) * FINAL_COST_MAP.get(curr);
                            board[dstRow][dstCol] = '.';
                            board[i][j] = curr;
                            ret.add(nextState);
                        }
                    }
                }
            }
        }
        // 第三部分 room -> room
//        for (int i = 0; i < 2; i++) {
//            for (int j = 2; j < 10; j += 2) {
//                char curr = board[i][j];
//                if (curr != '.') {
//                    // 如果是放好的，不动
//                    int dstCol = (curr - 'A') * 2 + 2;
//                    if (dstCol == j && (i == 0 || board[1 - i][j] == curr)) {
//                        continue;
//                    }
//                    if (dstCol != j) {
//                        boolean canPlaceRow0 = board[0][dstCol] == '.' && board[1][dstCol] == '.';
//                        boolean canPlaceRow1 = (board[0][dstCol] == curr && board[1][dstCol] == '.');
//                        boolean currRoomClear = i == 1 || board[1][j] == '.';
//                        boolean hallwayClear = hallwayClearHelper(board, j, dstCol);
//                        if (currRoomClear && hallwayClear) {
//                            int dstRow;
//                            if (canPlaceRow0) {
//                                dstRow = 0;
//                            } else if (canPlaceRow1) {
//                                dstRow = 1;
//                            } else {
//                                continue;
//                            }
//                            board[dstRow][dstCol] = curr;
//                            board[i][j] = '.';
//                            State nextState = new State();
//                            nextState.position = encode(board);
//                            int d = 2 - i + 2 - dstRow + Math.abs(j - dstCol);
//                            nextState.cost = cost + (long) d * FINAL_COST_MAP.get(curr);
//                            board[dstRow][dstCol] = '.';
//                            board[i][j] = curr;
//                            ret.add(nextState);
//                        }
//                    }
//                }
//            }
//        }
        return ret;
    }

    private boolean hallwayClearHelper(char[][] board, int i, int dstCol) {
        boolean hallwayClear = true;
        int lhs = Math.min(i, dstCol);
        int rhs = Math.max(i, dstCol);
        for (int k = lhs + 1; k < rhs; k++) {
            if (board[2][k] != '.') {
                hallwayClear = false;
                break;
            }
        }
        return hallwayClear;
    }
}
