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

struct Game {
    int n;
    vector<queue<int>> container_qs;
    vector<vector<int>> board;
    vector<Crane> cranes;
    vector<stack<int>> container_stacks;
    vector<vector<Operation>> history;
    vector<queue<Operation>> crane_operations;
    vector<int> requested;

    Game(int n) : n(n), container_qs(n), board(n, vector<int>(n, -1)), cranes(n), container_stacks(n), crane_operations(n), requested(n) {
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
        cranes[i].picking = board[cranes[i].col][cranes[i].row];
        board[cranes[i].col][cranes[i].row] = -1;
    }

    void move(int i, Operation dir) {
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
        board[cranes[i].col][cranes[i].row] = cranes[i].picking;
        cranes[i].picking = -1;
    }

    void crush(int i) {
        assert(cranes[i].picking == -1);
        cranes[i].is_crushed = true;
    }

    void tick() {
        rep(i, n) {
            if (board[i][0] == -1 && !container_qs[i].empty()) {
                board[i][0] = container_qs[i].front();
                container_qs[i].pop();
            }
        }
        rep(i, n) {
            if (board[i][n - 1] != -1) {
                container_stacks[i].push(board[i][n - 1]);
                board[i][n - 1] = -1;
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
            }
            history[i].push_back(op);
        }
    }

    pair<int, int> find_container(int v) {
        rep(i, n) {
            rep(j, n) {
                if (board[i][j] == v) {
                    return {i, j};
                }
            }
        }
        return {-1, -1};
    }

    vector<pair<int, int>> find_empty_arounds(pair<int, int> pos) {
        vector<pair<int, int>> empty_arounds;
        for (auto dir : directions) {
            int new_col = pos.first + dir.first;
            int new_row = pos.second + dir.second;
            if (new_row < 0 || new_row >= n || new_col < 0 || new_col >= n) continue;
            if (board[new_col][new_row] == -1) {
                empty_arounds.push_back({new_col, new_row});
            }
        }
        return empty_arounds;
    }

    void debug() {
        cerr << "DEBUG BOARD" << endl;
        rep(i, board.size()) {
            rep(j, board[i].size()) {
                cerr << board[i][j] << ' ';
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
                cerr << container_stacks_copy[i].top() << ' ';
                container_stacks_copy[i].pop();
            }
            cerr << endl;
        }
        cerr << "DEBUG HISTORY" << endl;
        rep(i, history.size()) {
            cerr << i << ": ";
            for (auto op : history[i]) {
                cerr << to_string(op) << ' ';
            }
            cerr << endl;
        }
        cerr << "END DEBUG" << endl;
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

    rep(i, in.n - 1) {
        game.crane_operations[i + 1].push(CRUSH);
    }

    while (true) {
        pair<int, int> crane_current = {game.cranes[0].col, game.cranes[0].row};

        vector<int> not_empty_cols;
        rep(i, in.n) {
            if (game.container_qs[i].empty()) continue;
            not_empty_cols.push_back(i);
            break;
        }

        if (game.crane_operations[0].empty() && !not_empty_cols.empty()) {
            int not_empty_col = not_empty_cols[0];
            pair<int, int> target_pos = {not_empty_col, 0};
            vector<pair<int, int>> empty_arounds = game.find_empty_arounds(target_pos);
            if (!empty_arounds.empty()) {
                vector<Operation> go_to_picking = get_path(crane_current, target_pos, PICKING);
                vector<Operation> go_to_releasing = get_path(target_pos, empty_arounds[0], RELEASING);
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
            vector<tuple<int, pair<int, int>, pair<int, int>>> scores;
            rep(i, in.n) {
                int request = game.requested[i];
                if (request == -1) continue;
                pair<int, int> target_pos = game.find_container(request);
                if (target_pos.first == -1) continue;
                int score = manhattan(crane_current, target_pos) + manhattan(target_pos, {i, in.n - 1});
                scores.push_back({score, target_pos, {i, in.n - 1}});
            }
            sort(scores.begin(), scores.end());
            if (scores.empty()) break;
            auto [score, target_pos_to_picking, target_pos_to_releasing] = scores[0];
            vector<Operation> go_to_pulling = get_path(crane_current, target_pos_to_picking, PICKING);
            vector<Operation> go_to_releasing = get_path(target_pos_to_picking, target_pos_to_releasing, RELEASING);
            for (auto& op : go_to_pulling) {
                game.crane_operations[0].push(op);
            }
            game.crane_operations[0].push(PICK);
            for (auto& op : go_to_releasing) {
                game.crane_operations[0].push(op);
            }
            game.crane_operations[0].push(RELEASE);
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
