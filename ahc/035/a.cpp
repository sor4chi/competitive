#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
typedef long long ll;
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
void println() { cout << '\n'; }
template <class T, class... Ts>
void eprintln(const T& a, const Ts&... b) {
    cerr << a;
    (void)(cerr << ... << (cerr << ' ', b));
    cerr << '\n';
}
template <class T>
void eprintv(const T& a, string sep = " ", string end = "\n") {
    for (auto x : a) {
        (void)(cerr << x << sep);
    }
    cerr << end;
}
void eprintln() { cerr << '\n'; }
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

unsigned long fast_rng() {
    static unsigned long x = 88172645463325252UL;
    x ^= x << 7;
    x ^= x >> 9;
    return x;
}
random_device seed_gen;
mt19937 engine(seed_gen());

void output_board(vector<vector<int>> board) {
    rep(i, board.size()) {
        rep(j, board[i].size()) {
            cout << board[i][j] << " ";
        }
        cout << endl;
    }
}

// double randnormal(std::mt19937& generator, normal_distribution<double>& distribution) {
//     return distribution(generator);
// }

// normal_distribution<double> distribution(0.0, 1.0);
// vector<int> generate_vector(int M) {
//     vector<double> x_prime(M);
//     double sum_squares = 0.0;

//     for (int i = 0; i < M; ++i) {
//         x_prime[i] = abs(randnormal(engine, distribution));
//         sum_squares += x_prime[i] * x_prime[i];
//     }

//     double pk = 100.0 / sqrt(sum_squares);
//     vector<int> x(M);

//     for (int i = 0; i < M; ++i) {
//         x[i] = round(pk * x_prime[i]);
//     }

//     return x;
// }

vector<int> generate_next_seed(vector<int> a, vector<int> b) {
    // 各要素ごとに等しい確率で選ぶ
    int m = a.size();
    vector<int> next_seed(m);
    rep(i, m) {
        int selected = rand() % 2;
        if (selected == 0) {
            next_seed[i] = a[i];
        } else {
            next_seed[i] = b[i];
        }
    }
    return next_seed;
}

vector<pair<int, int>> directions = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

vector<vector<int>> generate_next_seed_map(vector<vector<int>>& board, vector<vector<int>>& seed_map) {
    map<pair<int, int>, vector<int>> next_seeds;  // boardで隣接する全てのペアに対して、seedを生成する
    int n = board.size();
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            for (auto [dx, dy] : directions) {
                int ni = i + dx;
                int nj = j + dy;
                if (ni < 0 || ni >= n || nj < 0 || nj >= n) {
                    continue;
                }
                if (next_seeds.count({board[ni][nj], board[i][j]})) {
                    continue;
                }
                vector<int> a = seed_map[board[i][j]];
                eprintln("ni", ni, "nj", nj, "board[ni][nj]", board[ni][nj]);
                vector<int> b = seed_map[board[ni][nj]];
                vector<int> next_seed = generate_next_seed(a, b);
                next_seeds[{board[i][j], board[ni][nj]}] = next_seed;
            }
        }
    }
    vector<vector<int>> next_seed_map;
    for (auto [k, v] : next_seeds) {
        next_seed_map.push_back(v);
    }
    return next_seed_map;
}

int eval_seed_map(vector<vector<int>> seed_map) {
    // 全ての種の中で特徴量の総和が一番大きいものを選ぶ
    int max_score = 0;
    for (int i = 0; i < seed_map.size(); i++) {
        int score = 0;
        for (int j = 0; j < seed_map[i].size(); j++) {
            score += seed_map[i][j];
        }
        max_score = max(max_score, score);
    }
    return max_score;
}

vector<vector<int>> generate_next_board_in_greedy(vector<vector<int>> seed_map, int n) {
    vector<vector<int>> next_board(n, vector<int>(n));

    // 一番平均的にいいやつを選ぶ
    int max_score = 0;
    int max_seed = 0;
    for (int i = 0; i < seed_map.size(); i++) {
        int score = 0;
        for (int j = 0; j < seed_map[i].size(); j++) {
            score += seed_map[i][j];
        }
        if (score > max_score) {
            max_score = score;
            max_seed = i;
        }
    }

    // bfsする
    set<int> used;
    vector<vector<bool>> visited(n, vector<bool>(n));
    pair<int, int> start = {n / 2, n / 2};
    queue<pair<pair<int, int>, int>> q;
    q.push({start, max_seed});
    used.insert(max_seed);
    visited[start.first][start.second] = true;
    while (!q.empty()) {
        auto [p, seed] = q.front();
        q.pop();
        next_board[p.first][p.second] = seed;

        // // seedで特徴量が低いにソートする
        // vector<pair<int, int>> seed_features;
        // for (int i = 0; i < seed_map[seed].size(); i++) {
        //     seed_features.push_back({seed_map[seed][i], i});
        // }
        // sort(seed_features.begin(), seed_features.end());
        for (auto [dx, dy] : directions) {
            int ni = p.first + dx;
            int nj = p.second + dy;
            if (ni < 0 || ni >= n || nj < 0 || nj >= n) {
                continue;
            }
            if (visited[ni][nj]) {
                continue;
            }
            visited[ni][nj] = true;
            // // seedで特徴量が低い順に5つの合計が最も高いものを選ぶ
            // int max_score = INT_MIN;
            // int max_seed = 0;
            // for (int i = 0; i < seed_map.size(); i++) {
            //     if (used.count(i)) {
            //         continue;
            //     }
            //     int score = 0;
            //     for (int j = 0; j < 1; j++) {
            //         score += seed_features[j].first;
            //     }
            //     if (score > max_score) {
            //         max_score = score;
            //         max_seed = i;
            //     }
            // }

            // seedとの各特徴量の合計が一番大きいものを選ぶ
            int max_score = INT_MIN;
            int max_seed = 0;
            for (int i = 0; i < seed_map.size(); i++) {
                if (used.count(i)) {
                    continue;
                }
                int score = 0;
                for (int j = 0; j < seed_map[i].size(); j++) {
                    score += seed_map[i][j] + seed_map[seed][j];
                }
                if (score > max_score) {
                    max_score = score;
                    max_seed = i;
                }
            }
            q.push({{ni, nj}, max_seed});
            used.insert(max_seed);
        }
    }

    return next_board;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    int n, m, t;
    input(n, m, t);
    int t_len = 2 * n * (n - 1);
    vector<vector<int>> x(t_len, vector<int>(m));  // x -> seed_map
    rep(i, t_len) rep(j, m) input(x[i][j]);

    rep(ti, t) {
        // モンテカルロ法によって、boardのどこにどのseedを置くかを決定する
        vector<vector<int>> max_board(n, vector<int>(n));
        max_board = generate_next_board_in_greedy(x, n);
        // float max_score = 0;
        // chrono::system_clock::time_point start = chrono::system_clock::now();
        // int tl = 2000 / t + 1;
        // int iter = 0;
        // while (chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count() < tl) {
        //     iter++;
        //     vector<vector<int>> cur_x = x;
        //     vector<vector<int>> board(n, vector<int>(n));
        //     vector<int> perm(x.size());
        //     iota(perm.begin(), perm.end(), 0);
        //     shuffle(perm.begin(), perm.end(), engine);
        //     perm.resize(n * n);
        //     rep(i, n) rep(j, n) {
        //         board[i][j] = perm[i * n + j];
        //     }
        //     cur_x = generate_next_seed_map(board, x);
        //     // この後、boardにseedを貪欲に置いてnext_seedを生成する操作をt - ti回繰り返す
        //     // その後、boardのスコアを計算して、max_scoreを超えたら更新する
        //     rep(i, t - ti) {
        //         vector<vector<int>> next_board(n, vector<int>(n));
        //         next_board = generate_next_board_in_greedy(cur_x, n);
        //         cur_x = generate_next_seed_map(next_board, x);
        //     }
        //     if (eval_seed_map(cur_x) > max_score) {
        //         max_score = eval_seed_map(cur_x);
        //         max_board = board;
        //     }
        // }
        // eprintln("turn", ti, "iter", iter, "score", max_score);

        output_board(max_board);
        rep(i, t_len) rep(j, m) input(x[i][j]);
    }

    return 0;
}
