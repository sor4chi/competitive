#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
template <class T, class... Ts>
void print(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
}
template <class T, class... Ts>
void println(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
    cout << '\n';
}
template <class T>
void printv(const T& a, string sep = " ", string end = "\n") {
    for (auto x : a) {
        (void)(cout << x << sep);
    }
    cout << end;
}
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void println() { cout << '\n'; }
#define rep(i, n) for (int i = 0; i < n; i++)
#define rep1(i, n) for (int i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

struct Input {
    int n;
    vector<vector<int>> A;
};

struct Crane {
    int row, col;
    bool is_big;
    int picking;
    bool is_crushed;
};

enum Operation {
    STAY,     // stay
    PICK,     // pick
    UP,       // up
    DOWN,     // down
    LEFT,     // left
    RIGHT,    // right
    RELEASE,  // release
    CRUSH,    // crush
};

string to_string(Operation op) {
    if (op == PICK) {
        return "P";
    } else if (op == UP) {
        return "U";
    } else if (op == DOWN) {
        return "D";
    } else if (op == LEFT) {
        return "L";
    } else if (op == RIGHT) {
        return "R";
    } else if (op == RELEASE) {
        return "Q";
    } else if (op == CRUSH) {
        return "B";
    } else if (op == STAY) {
        return ".";
    }
    return "Invalid Operation";
}

int manhattan(pair<int, int> a, pair<int, int> b) {
    return abs(a.first - b.first) + abs(a.second - b.second);
}

const pair<int, int> directions[4] = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

enum PathMode {
    PICKING,
    RELEASING,
};

Operation rev_direction(Operation dir) {
    if (dir == UP) return DOWN;
    if (dir == DOWN) return UP;
    if (dir == LEFT) return RIGHT;
    if (dir == RIGHT) return LEFT;
    return STAY;
}

vector<Operation> get_path(pair<int, int> from, pair<int, int> to, PathMode mode) {
    vector<Operation> path;
    while (from != to) {
        if (mode == PICKING) {
            if (from.first < to.first) {
                path.push_back(DOWN);
                from.first++;
            } else if (from.first > to.first) {
                path.push_back(UP);
                from.first--;
            } else if (from.second < to.second) {
                path.push_back(RIGHT);
                from.second++;
            } else {
                path.push_back(LEFT);
                from.second--;
            }
        }
        if (mode == RELEASING) {
            if (from.second < to.second) {
                path.push_back(RIGHT);
                from.second++;
            } else if (from.second > to.second) {
                path.push_back(LEFT);
                from.second--;
            } else if (from.first < to.first) {
                path.push_back(DOWN);
                from.first++;
            } else {
                path.push_back(UP);
                from.first--;
            }
        }
    }
    return path;
}

vector<pair<int, int>> simulate_path(pair<int, int> from, vector<Operation> path) {
    vector<pair<int, int>> positions;
    positions.push_back(from);
    for (auto& op : path) {
        pair<int, int> next = positions.back();
        if (op == UP) {
            next.first--;
        } else if (op == DOWN) {
            next.first++;
        } else if (op == LEFT) {
            next.second--;
        } else if (op == RIGHT) {
            next.second++;
        }
        positions.push_back(next);
    }
    return positions;
}

struct Container {
    int value;
    int locker;
};

struct Game {
    int n;
    vector<queue<int>> container_qs;
    vector<vector<Container>> board;
    vector<Crane> cranes;
    vector<stack<int>> container_stacks;
    vector<vector<Operation>> history;
    vector<queue<Operation>> crane_operations;
    vector<int> requested;

    Game(int n) : n(n), container_qs(n), board(n, vector<Container>(n, {-1, -1})), cranes(n), container_stacks(n), crane_operations(n), requested(n) {
        rep(i, n) {
            cranes[i].row = 0;
            cranes[i].col = i;
            cranes[i].is_big = i == 0;
            cranes[i].picking = -1;
            cranes[i].is_crushed = false;
            requested[i] = i * n;
            crane_operations[i] = queue<Operation>();
        }
        history = vector<vector<Operation>>(n);
    }

    void add_container(int i, int v) {
        container_qs[i].push(v);
    }

    void pick(int i) {
        assert(cranes[i].picking == -1);
        assert(!cranes[i].is_crushed);
        assert(board[cranes[i].col][cranes[i].row].value != -1);
        assert(board[cranes[i].col][cranes[i].row].locker == i || board[cranes[i].col][cranes[i].row].locker == -1);
        cranes[i].picking = board[cranes[i].col][cranes[i].row].value;
        board[cranes[i].col][cranes[i].row].value = -1;
        board[cranes[i].col][cranes[i].row].locker = -1;
    }

    void move(int i, Operation dir) {
        assert(!cranes[i].is_crushed);
        if (dir == UP) {
            cranes[i].col--;
        } else if (dir == DOWN) {
            cranes[i].col++;
        } else if (dir == LEFT) {
            cranes[i].row--;
        } else if (dir == RIGHT) {
            cranes[i].row++;
        } else {
            cerr << "Invalid direction: " << dir << endl;
            assert(false);
        }
        if (cranes[i].row < 0 || cranes[i].row >= n || cranes[i].col < 0 || cranes[i].col >= n) {
            cerr << "Crane " << i << " moved out of the board" << endl;
            assert(false);
        }
    }

    void release(int i) {
        assert(cranes[i].picking != -1);
        assert(!cranes[i].is_crushed);
        board[cranes[i].col][cranes[i].row].value = cranes[i].picking;
        cranes[i].picking = -1;
    }

    void crush(int i) {
        assert(cranes[i].picking == -1);
        assert(!cranes[i].is_crushed);
        cranes[i].is_crushed = true;
    }

    void tick() {
        rep(i, n) {
            if (board[i][0].value == -1 && !container_qs[i].empty()) {
                board[i][0].value = container_qs[i].front();
                container_qs[i].pop();
            }
        }
        rep(i, n) {
            if (board[i][n - 1].value != -1) {
                container_stacks[i].push(board[i][n - 1].value);
                board[i][n - 1].value = -1;
            }
        }
        bool all_empty = true;
        rep(i, n) {
            if (!crane_operations[i].empty()) {
                all_empty = false;
                break;
            }
        }
        if (all_empty) return;
        rep(i, n) {
            Operation op;
            if (crane_operations[i].empty()) {
                op = STAY;
            } else {
                op = crane_operations[i].front();
                crane_operations[i].pop();
            }
            if (op == PICK) {
                pick(i);
            } else if (op == UP) {
                move(i, UP);
            } else if (op == DOWN) {
                move(i, DOWN);
            } else if (op == LEFT) {
                move(i, LEFT);
            } else if (op == RIGHT) {
                move(i, RIGHT);
            } else if (op == RELEASE) {
                release(i);
                if (cranes[i].row == n - 1) {
                    if (requested[cranes[i].col] == (cranes[i].col + 1) * n - 1) {
                        requested[cranes[i].col] = -1;
                    } else {
                        requested[cranes[i].col]++;
                    }
                }
            } else if (op == CRUSH) {
                crush(i);
                unlock_cranes_board(i);
            }
            history[i].push_back(op);
        }
    }

    void unlock_cranes_board(int i) {
        rep(j, n) {
            rep(k, n) {
                if (board[j][k].locker == i) {
                    board[j][k].locker = -1;
                }
            }
        }
    }

    int get_uncrushed_small_crane_id() {
        rep(i, n) {
            if (!cranes[i].is_crushed && !cranes[i].is_big) {
                return i;
            }
        }
        return -1;
    }

    bool is_request_completed() {
        rep(i, n) {
            if (requested[i] != -1) {
                return false;
            }
        }
        return true;
    }

    pair<int, int> find_container(int v) {
        rep(i, n) {
            rep(j, n) {
                if (board[i][j].value == v) {
                    return {i, j};
                }
            }
        }
        return {-1, -1};
    }

    vector<pair<int, int>> find_floating_containers() {
        set<pair<int, int>> floating_containers;
        rep(i, n) {
            rep(j, n - 2) {
                int revj = n - 2 - j;
                if (board[i][revj - 1].locker != -1 || board[i][revj].locker != -1) continue;
                if (board[i][revj - 1].value != -1 && board[i][revj].value == -1) {
                    floating_containers.insert({i, revj - 1});
                }
            }
        }
        vector<pair<int, int>> floating_containers_vec(floating_containers.begin(), floating_containers.end());
        return floating_containers_vec;
    }

    vector<pair<int, int>> find_empty_arounds(pair<int, int> pos) {
        vector<pair<int, pair<int, int>>> empty_arounds;
        rep(i, n) {
            rep(j, n - 1) {
                if (board[i][j].value == -1) {
                    int dist = manhattan(pos, {i, j});
                    empty_arounds.push_back({dist, {i, j}});
                }
            }
        }
        sort(empty_arounds.begin(), empty_arounds.end(), [](auto a, auto b) { return a.first < b.first; });
        vector<pair<int, int>> empty_arounds_vec;
        for (auto& ea : empty_arounds) {
            empty_arounds_vec.push_back(ea.second);
        }
        return empty_arounds_vec;
    }

    pair<int, int> peek_next_crane_pos(int i) {
        int next_row = cranes[i].row;
        int next_col = cranes[i].col;
        if (crane_operations[i].empty()) return {next_col, next_row};
        Operation next_op = crane_operations[i].front();
        if (next_op == UP) {
            next_col--;
        } else if (next_op == DOWN) {
            next_col++;
        } else if (next_op == LEFT) {
            next_row--;
        } else if (next_op == RIGHT) {
            next_row++;
        }
        return {next_col, next_row};
    }

    vector<Operation> crane_movable_operations(int i) {
        assert(!cranes[i].is_crushed);
        vector<Operation> movable_ops;
        if (cranes[i].col > 0) {
            movable_ops.push_back(UP);
        }
        if (cranes[i].col < n - 1) {
            movable_ops.push_back(DOWN);
        }
        if (cranes[i].row > 0) {
            movable_ops.push_back(LEFT);
        }
        if (cranes[i].row < n - 1) {
            movable_ops.push_back(RIGHT);
        }
        return movable_ops;
    }

    void debug() {
        cerr << endl;
        cerr << "=========================== DEBUG ===========================" << endl;
        cerr << "DEBUG BOARD VALUE" << endl;
        rep(i, board.size()) {
            rep(j, board[i].size()) {
                cerr << setw(2) << board[i][j].value << ' ';
            }
            cerr << endl;
        }
        cerr << "DEBUG BOARD LOCKER" << endl;
        rep(i, board.size()) {
            rep(j, board[i].size()) {
                cerr << setw(2) << board[i][j].locker << ' ';
            }
            cerr << endl;
        }
        cerr << "DEBUG CRANES" << endl;
        rep(i, cranes.size()) {
            cerr << "(" << cranes[i].col << "," << cranes[i].row << ") " << (cranes[i].is_big ? "bg" : "sm") << " picking: " << setw(2) << cranes[i].picking << " crushed: " << cranes[i].is_crushed << endl;
        }
        cerr << "DEBUG CONTAINER QS" << endl;
        vector<queue<int>> container_qs_copy = container_qs;
        rep(i, container_qs_copy.size()) {
            cerr << i << ": ";
            while (!container_qs_copy[i].empty()) {
                cerr << container_qs_copy[i].front() << ' ';
                container_qs_copy[i].pop();
            }
            cerr << endl;
        }
        cerr << "DEBUG CONTAINER STACKS" << endl;
        vector<stack<int>> container_stacks_copy = container_stacks;
        rep(i, container_stacks_copy.size()) {
            cerr << i << ": ";
            while (!container_stacks_copy[i].empty()) {
                cerr << setw(2) << container_stacks_copy[i].top() << ' ';
                container_stacks_copy[i].pop();
            }
            cerr << endl;
        }
        cerr << "DEBUG HISTORY" << endl;
        rep(i, history.size()) {
            for (auto op : history[i]) {
                cerr << to_string(op);
            }
            cerr << endl;
        }
    }
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    Input in;
    input(in.n);
    in.A.resize(in.n, vector<int>(in.n));
    rep(i, in.n) rep(j, in.n) input(in.A[i][j]);

    Game game(in.n);
    rep(i, in.n) rep(j, in.n) {
        game.add_container(i, in.A[i][j]);
    }
    game.tick();

    for (int width = in.n - 1; width >= 2; width--) {
        Operation op;
        int cnt = 0;
        while (cnt <= 2 * width - 1) {
            if (cnt == 0) op = PICK;
            if (cnt > 0 && cnt < width) op = RIGHT;
            if (cnt == width) op = RELEASE;
            if (cnt > width) op = LEFT;
            cnt++;
            rep(i, in.n) {
                if (op == LEFT && width == 2 && i == 0) continue;
                game.crane_operations[i].push(op);
            }
        }
    }

pull_end:

    int init_operation_count = game.crane_operations[0].size();
    int working_small_crane_id = -2;
    int iter = 0;
    int snapshot_stack_count = 0;
    vector<int> requested_snapshot = game.requested;
    while (!game.is_request_completed()) {
        bool same_snapshot = true;
        rep(i, in.n) {
            if (game.requested[i] != requested_snapshot[i]) {
                same_snapshot = false;
                break;
            }
        }
        if (same_snapshot) {
            snapshot_stack_count++;
        } else {
            snapshot_stack_count = 0;
        }
        requested_snapshot = game.requested;
        iter++;

        vector<int> uncrashed_cranes;
        rep(i, in.n) {
            if (!game.cranes[i].is_crushed) {
                uncrashed_cranes.push_back(i);
            }
        }

        bool is_qs_empty = true;
        rep(i, in.n) {
            if (!game.container_qs[i].empty()) {
                is_qs_empty = false;
                break;
            }
        }

        if (game.crane_operations[0].empty() && uncrashed_cranes.size() == 1 && !is_qs_empty && snapshot_stack_count > 10) {
            int col = -1;
            rep(i, in.n) {
                if (game.board[i][0].value != -1) {
                    col = i;
                    break;
                }
            }
            if (col != -1) {
                pair<int, int> crane_current = {game.cranes[0].col, game.cranes[0].row};
                pair<int, int> target_pos = {col, 0};
                vector<Operation> go_to_picking = get_path(crane_current, target_pos, PICKING);
                vector<Operation> go_to_releasing = get_path(target_pos, {target_pos.first, target_pos.second + 1}, RELEASING);
                game.board[target_pos.first][target_pos.second].locker = 0;
                for (auto& op : go_to_picking) {
                    game.crane_operations[0].push(op);
                }
                game.crane_operations[0].push(PICK);
                for (auto& op : go_to_releasing) {
                    game.crane_operations[0].push(op);
                }
                game.crane_operations[0].push(RELEASE);
            }
        }

        if (game.crane_operations[0].empty()) {
            pair<int, int> crane_current = {game.cranes[0].col, game.cranes[0].row};
            vector<tuple<int, pair<int, int>, pair<int, int>>> scores;
            rep(i, in.n) {
                int request = game.requested[i];
                if (request == -1) continue;
                pair<int, int> target_pos = game.find_container(request);
                if (target_pos.first == -1) continue;
                if (game.board[target_pos.first][target_pos.second].locker != -1) continue;
                int score = manhattan(crane_current, target_pos) + manhattan(target_pos, {i, in.n - 1});
                scores.push_back({score, target_pos, {i, in.n - 1}});
            }
            sort(scores.begin(), scores.end());
            if (scores.empty()) goto skip_big_crane;
            auto [score, target_pos_to_picking, target_pos_to_releasing] = scores[0];
            vector<Operation> go_to_pulling = get_path(crane_current, target_pos_to_picking, PICKING);
            vector<Operation> go_to_releasing = get_path(target_pos_to_picking, target_pos_to_releasing, RELEASING);
            game.board[target_pos_to_picking.first][target_pos_to_picking.second].locker = 0;
            for (auto& op : go_to_pulling) {
                game.crane_operations[0].push(op);
            }
            game.crane_operations[0].push(PICK);
            for (auto& op : go_to_releasing) {
                game.crane_operations[0].push(op);
            }
            game.crane_operations[0].push(RELEASE);
        }

    skip_big_crane:

        vector<pair<int, int>> floating_containers = game.find_floating_containers();
        if (!floating_containers.empty() && working_small_crane_id != -1 && iter > init_operation_count) {
            pair<int, int> container = floating_containers[0];
            if (working_small_crane_id == -2) {
                int closest_small_crane_id = -1;
                int min_dist = 1e9;
                rep(i, in.n) {
                    if (game.cranes[i].is_big) continue;
                    if (game.cranes[i].is_crushed) continue;
                    int dist = manhattan({game.cranes[i].col, game.cranes[i].row}, container);
                    if (dist < min_dist) {
                        min_dist = dist;
                        closest_small_crane_id = i;
                    }
                }
                working_small_crane_id = closest_small_crane_id;
            }
            if (game.cranes[working_small_crane_id].is_crushed) {
                working_small_crane_id = game.get_uncrushed_small_crane_id();
                if (working_small_crane_id == -1) {
                    goto small_container_end;
                }
            }
            if (game.crane_operations[working_small_crane_id].empty()) {
                pair<int, int> crane_current = {game.cranes[working_small_crane_id].col, game.cranes[working_small_crane_id].row};
                pair<int, int> target_pos = container;
                vector<Operation> go_to_picking = get_path(crane_current, target_pos, PICKING);
                vector<Operation> go_to_releasing = get_path(target_pos, {target_pos.first, target_pos.second + 1}, RELEASING);
                game.board[target_pos.first][target_pos.second].locker = working_small_crane_id;
                for (auto& op : go_to_picking) {
                    game.crane_operations[working_small_crane_id].push(op);
                }
                game.crane_operations[working_small_crane_id].push(PICK);
                for (auto& op : go_to_releasing) {
                    game.crane_operations[working_small_crane_id].push(op);
                }
                game.crane_operations[working_small_crane_id].push(RELEASE);
            };
        }

    small_container_end:

        set<int> tmp_crushed;
        rep(i, in.n) {
            for (int j = i + 1; j < in.n; j++) {
                if (i == j) continue;
                if (game.cranes[i].is_crushed || game.cranes[j].is_crushed) continue;
                if (tmp_crushed.count(i) || tmp_crushed.count(j)) continue;
                pair<int, int> current_i_pos = {game.cranes[i].col, game.cranes[i].row};
                pair<int, int> current_j_pos = {game.cranes[j].col, game.cranes[j].row};
                pair<int, int> next_i_pos = game.peek_next_crane_pos(i);
                pair<int, int> next_j_pos = game.peek_next_crane_pos(j);
                if (next_i_pos == next_j_pos) {
                    if (game.cranes[j].picking != -1) {
                        vector<pair<int, int>> empty_arounds = game.find_empty_arounds(current_j_pos);
                        vector<Operation> best_path;
                        for (auto& ea : empty_arounds) {
                            vector<Operation> path = get_path(current_j_pos, ea, RELEASING);
                            vector<pair<int, int>> positions = simulate_path(current_j_pos, path);
                            bool found = true;
                            for (auto& pos : positions) {
                                if (game.board[pos.first][pos.second].value != -1) {
                                    found = false;
                                    break;
                                }
                                if (pos == current_i_pos) {
                                    found = false;
                                    break;
                                }
                            }
                            if (found) {
                                best_path = path;
                                break;
                            }
                        }
                        queue<Operation> tmp;
                        tmp.push(STAY);
                        while (!game.crane_operations[i].empty()) {
                            tmp.push(game.crane_operations[i].front());
                            game.crane_operations[i].pop();
                        }
                        game.crane_operations[i] = tmp;
                        game.crane_operations[j] = queue<Operation>();
                        game.unlock_cranes_board(j);
                        for (auto& op : best_path) {
                            game.crane_operations[j].push(op);
                        }
                        game.crane_operations[j].push(RELEASE);
                        game.crane_operations[j].push(CRUSH);
                    } else {
                        if (game.cranes[j].picking == -1) {
                            game.crane_operations[j] = queue<Operation>();
                            game.crane_operations[j].push(CRUSH);
                            game.unlock_cranes_board(j);
                            tmp_crushed.insert(j);
                        } else {
                            queue<Operation> tmp;
                            tmp.push(STAY);
                            while (!game.crane_operations[j].empty()) {
                                tmp.push(game.crane_operations[j].front());
                                game.crane_operations[j].pop();
                            }
                            game.crane_operations[j] = tmp;
                        }
                    }
                }
                if (current_i_pos == next_j_pos && current_j_pos == next_i_pos) {
                    vector<Operation> movable = game.crane_movable_operations(i);
                    Operation next_move = game.crane_operations[i].front();
                    Operation tmp;
                    for (auto m : movable) {
                        if (m != next_move) {
                            tmp = m;
                            break;
                        }
                    }
                    queue<Operation> tmpq;
                    tmpq.push(tmp);
                    tmpq.push(STAY);
                    tmpq.push(rev_direction(tmp));
                    while (!game.crane_operations[i].empty()) {
                        tmpq.push(game.crane_operations[i].front());
                        game.crane_operations[i].pop();
                    }
                    game.crane_operations[i] = tmpq;
                }
            }
        }

        game.tick();
    }

    for (auto& a : game.history) {
        for (auto& b : a) {
            cout << to_string(b);
        }
        cout << '\n';
    }

    return 0;
}
