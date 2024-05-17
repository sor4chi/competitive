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

struct Output {
    vector<vector<char>> ans;
};

struct Container {
    int v, col;
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

    Output out;
    out.ans.resize(in.n);

    auto get_path = [&](pair<int, int> from, pair<int, int> to) {
        vector<char> path;
        while (from != to) {
            if (from.first < to.first) {
                path.push_back('D');
                from.first++;
            } else if (from.first > to.first) {
                path.push_back('U');
                from.first--;
            } else if (from.second < to.second) {
                path.push_back('R');
                from.second++;
            } else {
                path.push_back('L');
                from.second--;
            }
        }
        return path;
    };

    pair<int, int> cur = {0, 0};

    vector<vector<int>> board(in.n, vector<int>(in.n, -1));

    auto debug_board = [&]() {
        cerr << "board:\n";
        rep(i, in.n) {
            rep(j, in.n) {
                cerr << setw(2) << setfill(' ') << board[i][j] << ' ';
            }
            cerr << '\n';
        }
    };

    debug_board();

    rep(i, in.n) {
        rep(j, in.n - 1) {
            out.ans[i].push_back('P');
            rep(k, in.n - j - 2) {
                out.ans[i].push_back('R');
            }
            out.ans[i].push_back('Q');
            rep(k, in.n - j - 2) {
                out.ans[i].push_back('L');
            }
            board[i][in.n - j - 2] = in.A[i][j];
        }
        out.ans[i].pop_back();
        out.ans[i].pop_back();
    }

    debug_board();

    auto find_value_from_board = [&](int v) {
        rep(i, in.n) {
            rep(j, in.n) {
                if (board[i][j] == v) {
                    return make_pair(i, j);
                }
            }
        }
        return make_pair(-1, -1);
    };

    vector<int> request(in.n);
    auto debug_request = [&]() {
        cerr << "request: ";
        rep(i, in.n) {
            cerr << request[i] << ' ';
        }
        cerr << '\n';
    };

    rep(i, in.n) {
        request[i] = in.n * i;
    }

    rep(i, in.n - 1) {
        out.ans[i + 1].push_back('B');
    }

    auto is_request_cleared = [&]() {
        vector<int> request_copy = request;
        sort(request_copy.begin(), request_copy.end());
        rep(i, in.n) {
            if (request_copy[i] != (i + 1) * 5) {
                return false;
            }
        }
        return true;
    };

restart:;
    int q = in.n * in.n;
    while (q-- && !is_request_cleared()) {
        rep(i, 100000000) {
        }
        rep(i, in.n) {
            if (request[i] == -1) continue;
            auto p = find_value_from_board(request[i]);
            if (p.first == -1 && p.second == -1) continue;
            cerr << "request: " << request[i] << " found: " << p.first << ' ' << p.second << '\n';
            auto path = get_path(cur, p);
            for (auto& c : path) {
                out.ans[0].push_back(c);
            }
            cur = p;
            auto path2 = get_path(p, {request[i] / in.n, in.n - 1});
            out.ans[0].push_back('P');
            for (auto& c : path2) {
                out.ans[0].push_back(c);
            }
            out.ans[0].push_back('Q');
            board[p.first][p.second] = -1;
            cerr << "clear: " << request[i] << "|" << i << '\n';
            cur = {request[i] / in.n, in.n - 1};
            request[i]++;
            if (p.second == 0) {
                board[p.first][0] = in.A[p.first][in.n - 1];
                in.A[p.first][in.n - 1] = -1;
            }
            break;
        }
        debug_request();
        debug_board();
    }

    if (!is_request_cleared()) {
        rep(i, in.n) {
            if (board[i][0] != -1) {
                pair<int, int> to;
                rep(i, in.n) {
                    rep(j, in.n - 2) {
                        if (board[i][j + 1] == -1) {
                            to = {i, j + 1};
                        }
                    }
                }
                auto path_from = get_path(cur, {i, 0});
                for (auto& c : path_from) {
                    out.ans[0].push_back(c);
                }
                cur = {i, 0};
                auto path_to = get_path({i, 0}, to);
                out.ans[0].push_back('P');
                for (auto& c : path_to) {
                    out.ans[0].push_back(c);
                }
                out.ans[0].push_back('Q');
                board[to.first][to.second] = board[i][0];
                board[i][0] = in.A[i][in.n - 1];
                in.A[i][in.n - 1] = -1;
                cur = to;
            }
        }
        debug_board();
        goto restart;
    }

    for (auto& a : out.ans) {
        for (auto& b : a) {
            cout << b;
        }
        cout << '\n';
    }

    return 0;
}
