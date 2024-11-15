#define DEBUG false

#if DEBUG
#else
#pragma GCC optimize "O3"
#pragma GCC target "sse4.2"
#endif

#include <string.h>

#include <algorithm>
#include <bitset>
#include <cassert>
#include <chrono>
#include <iostream>
#include <map>
#include <random>
#include <stack>
#include <vector>

using namespace std;
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

// 時計回りに8方向の矢印
enum Arrow {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft
};

static unsigned long x = 123456789, y = 362436069, z = 521288629;

// eijiroさんのコードを拝借
class Xorshift {
   public:
    explicit Xorshift(uint32_t seed) : x_(seed) {
        assert(seed);
    }

    // [0, stop)
    uint32_t randrange(uint32_t stop) {
        assert(stop > 0);
        next();
        return x_ % stop;
    }

    // [start, stop)
    uint32_t randrange(uint32_t start, uint32_t stop) {
        assert(start < stop);
        next();
        return start + x_ % (stop - start);
    }

    // [a, b]
    uint32_t randint(uint32_t a, uint32_t b) {
        assert(a <= b);
        return randrange(a, b + 1);
    }

    // [0.0, 1.0]
    double random() {
        next();
        return static_cast<double>(x_) * (1.0 / static_cast<double>(UINT32_MAX));
    }

    // [a, b] or [b, a]
    double uniform(double a, double b) {
        return a + (b - a) * random();
    }

    uint32_t raw() {
        next();
        return x_;
    }

   private:
    uint32_t x_;

    void next() {
        x_ ^= x_ << 13;
        x_ ^= x_ >> 17;
        x_ ^= x_ << 5;
    }
};

map<Arrow, pair<int, int>> dirs = {
    {Top, {-1, 0}},
    {TopRight, {-1, 1}},
    {Right, {0, 1}},
    {BottomRight, {1, 1}},
    {Bottom, {1, 0}},
    {BottomLeft, {1, -1}},
    {Left, {0, -1}},
    {TopLeft, {-1, -1}}};

map<Arrow, Arrow> revs = {
    {Top, Bottom},
    {TopRight, BottomLeft},
    {Right, Left},
    {BottomRight, TopLeft},
    {Bottom, Top},
    {BottomLeft, TopRight},
    {Left, Right},
    {TopLeft, BottomRight}};

struct Cell {
    Arrow arrow;
    int mult;
};

int n;
map<int, int> mults;
vector<vector<Cell>> grid;
int all_tl = 9800;
chrono::system_clock::time_point all_start = chrono::system_clock::now();

#if DEBUG
struct Plotter {
    vector<pair<string, vector<int>>> buffer;

    void add(const vector<int>& row, string label = "") {
        buffer.push_back({label, row});
    }

    void plot() {
        eprintln("plotting...");
        system("rm -f plot.png");
        string command = "python3 -c 'import matplotlib.pyplot as plt\n";
        command += "import numpy as np\n";
        // buffer.size()個のグラフを重ねて描画
        command += "fig, ax = plt.subplots()\n";
        for (int i = 0; i < buffer.size(); i++) {
            command += "ax.plot(np.array(";
            command += "[";
            for (int j = 0; j < buffer[i].second.size(); j++) {
                command += to_string(buffer[i].second[j]);
                if (j != buffer[i].second.size() - 1) {
                    command += ",";
                }
            }
            command += "]), label=\"";
            command += buffer[i].first;
            command += "\")\n";
        }
        command += "plt.legend()\n";
        command += "plt.xlabel(\"iteration (x100)\")\n";
        command += "plt.ylabel(\"score\")\n";
        command += "plt.savefig(\"plot.png\")'";
        system(command.c_str());
    }
};

Plotter plotter;
#endif

struct DFSNode {
    int r;
    int c;
    int i;
    int score;
    vector<pair<int, int>> moves;
    bitset<900> used;
};

struct NextDFSNode {
    DFSNode node;
    int point;
    int far;
};

pair<vector<pair<int, int>>, int> dfs_greedy(int r, int c, int tl) {
    int n = grid.size();
    bitset<900> used;
    used[r * n + c] = true;
    vector<pair<int, int>> best_moves;
    int best_score = 0;
    stack<DFSNode> st;
    vector<pair<int, int>> initial_moves;
    initial_moves.push_back({r, c});
    DFSNode initial = {r, c, 1, grid[r][c].mult, initial_moves, used};
    st.push(initial);
    chrono::system_clock::time_point start = chrono::system_clock::now();

    while (!st.empty() && chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count() < tl) {
        auto [r, c, i, score, moves, used] = st.top();
        st.pop();

        if (score > best_score) {
            best_score = score;
            best_moves = moves;
        }

        auto [arrow, m] = grid[r][c];
        auto [dr, dc] = dirs[arrow];

        vector<NextDFSNode> nexts;
        int r_ = r;
        int c_ = c;
        int far = 0;
        while (true) {
            r_ += dr;
            c_ += dc;
            if (r_ < 0 || r_ >= n || c_ < 0 || c_ >= n) {
                break;
            }
            if (used[r_ * n + c_]) {
                continue;
            }
            bitset<900> used_ = used;
            used_[r_ * n + c_] = true;
            vector<pair<int, int>> moves_ = moves;
            moves_.push_back({r_, c_});
            DFSNode next = {r_, c_, i + 1, score + grid[r_][c_].mult * (i + 1), moves_, used_};
            nexts.push_back({next, grid[r_][c_].mult, far++});
        }

        int cap = mults[1] * 0.75 + mults[2] * 0.5 + mults[3] * 0.125;
        bool should_point_maximize = i >= cap;

        sort(nexts.begin(), nexts.end(), [should_point_maximize](const NextDFSNode& a, const NextDFSNode& b) {
            // スコアが小さい(pointが低い方)から選ぶ
            // (後に高いマスを踏んだ方がおいしい)
            if (a.point != b.point) {
                if (should_point_maximize) {
                    return a.point < b.point;
                }
                return a.point > b.point;
            }
            // 近いマスから選ぶ
            return a.far > b.far;
        });

        for (auto& next : nexts) {
            st.push(next.node);
        }
    }

    return {best_moves, best_score};
}

pair<vector<pair<int, int>>, int> random_dfs_greedy(int r, int c, int tl, Xorshift& rng) {
    int n = grid.size();
    bitset<900> used;
    used[r * n + c] = true;
    vector<pair<int, int>> best_moves;
    int best_score = 0;
    stack<DFSNode> st;
    vector<pair<int, int>> initial_moves;
    initial_moves.push_back({r, c});
    DFSNode initial = {r, c, 1, grid[r][c].mult, initial_moves, used};
    st.push(initial);
    chrono::system_clock::time_point start = chrono::system_clock::now();
    mt19937 mt(rng.raw());

    while (!st.empty() && chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count() < tl) {
        auto [r, c, i, score, moves, used] = st.top();
        st.pop();

        if (score > best_score) {
            best_score = score;
            best_moves = moves;
        }

        auto [arrow, m] = grid[r][c];
        auto [dr, dc] = dirs[arrow];

        vector<NextDFSNode> nexts;
        int r_ = r;
        int c_ = c;
        int far = 0;
        while (true) {
            r_ += dr;
            c_ += dc;
            if (r_ < 0 || r_ >= n || c_ < 0 || c_ >= n) {
                break;
            }
            if (used[r_ * n + c_]) {
                continue;
            }
            bitset<900> used_ = used;
            used_[r_ * n + c_] = true;
            vector<pair<int, int>> moves_ = moves;
            moves_.push_back({r_, c_});
            DFSNode next = {r_, c_, i + 1, score + grid[r_][c_].mult * (i + 1), moves_, used_};
            nexts.push_back({next, grid[r_][c_].mult, far++});
        }

        shuffle(nexts.begin(), nexts.end(), mt);

        for (auto& next : nexts) {
            st.push(next.node);
        }
    }

    return {best_moves, best_score};
}

int eval(const vector<short>& moves) {
    int score = 0;
    rep(i, moves.size()) {
        auto [r, c] = make_pair(moves[i] / n, moves[i] % n);
        score += grid[r][c].mult * (i + 1);
    }
    return score;
}

bool validate(const vector<pair<int, int>>& moves) {
    if (moves.size() > n * n) {
        return false;
    }
    // usedの検証とarrowに従っているかの検証
    bitset<900> used;
    rep(i, moves.size()) {
        auto [r, c] = moves[i];
        if (used[r * n + c]) {
            return false;
        }
        used[r * n + c] = true;
        if (i == 0) {
            continue;
        }
        auto [r_, c_] = moves[i - 1];
        auto [arrow, m] = grid[r_][c_];
        auto [dr, dc] = dirs[arrow];
        bool ok = false;
        while (true) {
            r_ += dr;
            c_ += dc;
            if (r_ < 0 || r_ >= n || c_ < 0 || c_ >= n) {
                break;
            }
            if (r_ == r && c_ == c) {
                ok = true;
                break;
            }
        }
        if (!ok) {
            return false;
        }
    }
    return true;
}

enum Neighbor {
    Break,
    ExpandStart,
    ExpandEnd,
    BreakStart,
    BreakEnd,
};

map<Neighbor, int> neighbor_weights = {
    {Break, 5},
    {ExpandStart, 1},
    {ExpandEnd, 1},
    // {BreakStart, 1},
    // {BreakEnd, 1},
};

vector<Neighbor> choice_neighbors;

Neighbor neighbor(Xorshift& rng) {
    if (choice_neighbors.empty()) {
        for (auto [neighbor, weight] : neighbor_weights) {
            rep(i, weight) {
                choice_neighbors.push_back(neighbor);
            }
        }
    }
    return choice_neighbors[rng.randrange(choice_neighbors.size())];
}

// 山登り法で最適解を探す
pair<vector<pair<int, int>>, int> hill_climbing(int tl, int trial) {
    Xorshift rng(trial);
    chrono::system_clock::time_point start = chrono::system_clock::now();

    short best_moves[900];
    int best_move_size = 0;
    int best_score = 0;
    while (true) {
        // まずはランダムなスタート地点を選ぶ
        int r = rng.randint(0, n - 1);
        int c = rng.randint(0, n - 1);
        // DFSで初期解を求める
        auto [moves, score] = random_dfs_greedy(r, c, 1, rng);
        if (moves.size() >= 3) {
            best_move_size = moves.size();
            for (int i = 0; i < best_move_size; i++) {
                auto [r, c] = moves[i];
                best_moves[i] = r * n + c;
            }
            best_score = score;
            break;
        }
    }

    bitset<900> initial_used;
    for (int i = 0; i < best_move_size; i++) {
        initial_used[best_moves[i]] = true;
    }

    short current_moves[900] = {};
    memcpy(current_moves, best_moves, sizeof(short) * best_move_size);
    int current_move_size = best_move_size;
    bitset<900> current_used = initial_used;
    int current_score = best_score;

    // 部分破壊 -> 再構築操作を繰り返す
    // 焼きなまし
    double start_temp = 20 * powf(n, 1.25);
    double end_temp = 0;
    int updates = 0;
    int iters = 0;
    mt19937 mt(rng.raw());

    vector<int> score_history;

    while (chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count() < tl) {
        iters++;
        short new_moves[900] = {};
        int new_move_size = 0;
        bitset<900> new_used;
        Neighbor nb = neighbor(rng);
        if (nb == Break) {
            // 一部をランダムに破壊
            int l = rng.randint(1, current_move_size - 1);
            int width = rng.randint(1, 8);
            int r = min(current_move_size - 1, l + width);
            auto broken_used = current_used;
            short start_cell = current_moves[l - 1];
            short end_cell = current_moves[r];

            for (int i = l; i <= r; i++) {
                auto move = current_moves[i];
                broken_used[move] = false;
            }

            // startとendの間のマスをdfsで探索し、今よりもさらに長い操作列を求める
            short founded_moves[900] = {};
            int founded_move_size = 0;
            struct HCDFSNode {
                short pos;
                short moves[900];
                int move_size;
                bitset<900> used;
            };
            stack<HCDFSNode> st;
            short initial_moves[900] = {start_cell};
            st.push({start_cell, {}, 1, broken_used});
            memcpy(st.top().moves, initial_moves, sizeof(short));
            // 今最適解よりも長い操作列が見つかったら、それを最適解とする
            chrono::system_clock::time_point start_dfs = chrono::system_clock::now();
            int tl_dfs = 10;
            while (!st.empty() && chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start_dfs).count() < tl_dfs) {
                auto [cur_pos, cur_moves, cur_move_size, cur_used] = st.top();
                st.pop();
                if (cur_pos == end_cell) {
                    memcpy(founded_moves, cur_moves, sizeof(short) * cur_move_size);
                    founded_move_size = cur_move_size;
                    new_used = cur_used;
                    break;
                }
                auto [cur_r, cur_c] = make_pair(cur_pos / n, cur_pos % n);
                auto [arrow, m] = grid[cur_r][cur_c];
                auto [dr, dc] = dirs[arrow];
                int r_ = cur_r;
                int c_ = cur_c;
                vector<HCDFSNode> nexts;
                while (true) {
                    r_ += dr;
                    c_ += dc;
                    if (r_ < 0 || r_ >= n || c_ < 0 || c_ >= n) {
                        break;
                    }
                    if (cur_used[r_ * n + c_]) {
                        continue;
                    }
                    auto used_ = cur_used;
                    used_[r_ * n + c_] = true;
                    short moves_[900] = {};
                    memcpy(moves_, cur_moves, sizeof(short) * cur_move_size);
                    moves_[cur_move_size] = r_ * n + c_;
                    HCDFSNode next = {(short)(r_ * n + c_), {}, cur_move_size + 1, used_};
                    memcpy(next.moves, moves_, sizeof(short) * (cur_move_size + 1));
                    nexts.push_back(next);
                }

                shuffle(nexts.begin(), nexts.end(), mt);

                for (auto& next : nexts) {
                    st.push(next);
                }
            }

            if (founded_move_size == 0) {
                continue;
            }

            // 最も長い操作列を求めたので、それが既存のパスより長い場合は更新する

            for (int i = 0; i < l - 1; i++) {
                new_moves[new_move_size++] = current_moves[i];
            }
            for (int i = 0; i < founded_move_size; i++) {
                new_moves[new_move_size++] = founded_moves[i];
            }
            for (int i = r + 1; i < current_move_size; i++) {
                new_moves[new_move_size++] = current_moves[i];
            }
        }

        if (nb == ExpandStart) {
            // スタート地点からrev方面に一マス拡張
            short start_cell = current_moves[0];
            auto [r, c] = make_pair(start_cell / n, start_cell % n);
            vector<short> candidates;
            for (auto [arrow, dir] : dirs) {
                int r_ = r;
                int c_ = c;
                auto [dr, dc] = dir;
                while (true) {
                    r_ += dr;
                    c_ += dc;
                    if (r_ < 0 || r_ >= n || c_ < 0 || c_ >= n) {
                        break;
                    }
                    if (current_used[r_ * n + c_]) {
                        continue;
                    }
                    if (revs[arrow] == grid[r_][c_].arrow) {
                        candidates.push_back(r_ * n + c_);
                    }
                }
            }

            if (candidates.empty()) {
                continue;
            }

            short new_start = candidates[rng.randrange(candidates.size())];
            new_moves[new_move_size++] = new_start;
            for (int i = 0; i < current_move_size; i++) {
                new_moves[new_move_size++] = current_moves[i];
            }
            new_used = current_used;
            new_used[new_start] = true;
        }

        if (nb == ExpandEnd) {
            // ゴール地点からrev方面に一マス拡張
            short end_cell = current_moves[current_move_size - 1];
            auto [r, c] = make_pair(end_cell / n, end_cell % n);
            vector<short> candidates;
            int r_ = r;
            int c_ = c;
            auto [arrow, m] = grid[r][c];
            auto [dr, dc] = dirs[arrow];
            while (true) {
                r_ += dr;
                c_ += dc;
                if (r_ < 0 || r_ >= n || c_ < 0 || c_ >= n) {
                    break;
                }
                if (current_used[r_ * n + c_]) {
                    continue;
                }
                if (revs[arrow] == grid[r_][c_].arrow) {
                    candidates.push_back(r_ * n + c_);
                }
            }

            if (candidates.empty()) {
                continue;
            }

            short new_end = candidates[rng.randrange(candidates.size())];
            for (int i = 0; i < current_move_size; i++) {
                new_moves[i] = current_moves[i];
            }
            new_moves[new_move_size++] = new_end;
            new_used = current_used;
            new_used[new_end] = true;
        }

        if (nb == BreakStart) {
            for (int i = 1; i < current_move_size; i++) {
                new_moves[new_move_size++] = current_moves[i];
            }
            new_used = current_used;
            new_used[current_moves[0]] = false;
        }

        if (nb == BreakEnd) {
            for (int i = 0; i < current_move_size - 1; i++) {
                new_moves[new_move_size++] = current_moves[i];
            }
            new_used = current_used;
            new_used[current_moves[current_move_size - 1]] = false;
        }

        if (new_move_size <= 3) {
            continue;
        }

        int new_score = eval({new_moves, new_moves + new_move_size});

        int diff = new_score - current_score;
        chrono::system_clock::time_point now = chrono::system_clock::now();
        int progress = chrono::duration_cast<chrono::milliseconds>(now - start).count();
        double temp = start_temp + (double)((end_temp - start_temp) * progress) / (double)tl;
        // 対数的に温度を下げる
        // double temp = start_temp * pow(end_temp / start_temp, (double)progress / tl);
        double prob = exp(diff / temp);

        if (diff > 0 || prob > rng.random()) {
#if DEBUG
            if (updates % 10 == 0) {
                score_history.push_back(current_score);
            }
#endif
            memcpy(current_moves, new_moves, sizeof(short) * new_move_size);
            current_move_size = new_move_size;
            current_score = new_score;
            current_used = new_used;
            updates++;
        }

        if (current_score > best_score) {
            memcpy(best_moves, current_moves, sizeof(short) * current_move_size);
            best_move_size = current_move_size;
            best_score = current_score;
        }
    }

    eprintln("updates", updates, "iters", iters);
#if DEBUG
    plotter.add(score_history, "trial" + to_string(trial));
#endif

    vector<pair<int, int>> best_moves_;
    for (int i = 0; i < best_move_size; i++) {
        auto move = best_moves[i];
        best_moves_.push_back({move / n, move % n});
    }

    return {best_moves_, best_score};
}

int main() {
    all_start = chrono::system_clock::now();
    input(n);
    grid = vector<vector<Cell>>(n, vector<Cell>(n));
    mults[1] = 0;
    mults[2] = 0;
    mults[3] = 0;
    mults[5] = 0;
    rep(r, n) {
        rep(c, n) {
            int a, m;
            input(a, m);
            grid[r][c] = {Arrow(a), m};
            mults[m]++;
        }
    }

    vector<pair<int, int>> best_moves;
    int best_score = 0;

    struct Cand {
        int mult;
        int dist_from_center;
        pair<int, int> pos;
    };

    vector<Cand> start_candidates;
    rep(r, n) {
        rep(c, n) {
            int dist = abs(r - n / 2) + abs(c - n / 2);
            Cand cand = {grid[r][c].mult, dist, {r, c}};
            start_candidates.push_back(cand);
        }
    }

    int trial = 5;
    int each_tl = 1950;
    rep(i, trial) {
        auto [moves, score] = hill_climbing(each_tl, i + 1);
        eprintln("trial", i + 1, "score", score);
        if (score > best_score) {
            best_moves = moves;
            best_score = score;
        }
    }

    println(best_moves.size());
    for (auto [r, c] : best_moves) {
        println(r, c);
    }

#if DEBUG
    plotter.plot();
#endif

    return 0;
}
