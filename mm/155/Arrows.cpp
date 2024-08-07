#pragma GCC target "sse4.2"
#include <bits/stdc++.h>

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

unsigned long xorshf96(void) {
    unsigned long t;
    x ^= x << 16;
    x ^= x >> 5;
    x ^= x << 1;

    t = x;
    x = y;
    y = z;
    z = t ^ x ^ y;

    return z;
}

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

int eval(const vector<pair<int, int>>& moves) {
    int score = 0;
    rep(i, moves.size()) {
        auto [r, c] = moves[i];
        auto [_, m] = grid[r][c];
        score += m * (i + 1);
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
    {Break, 1},
    {ExpandStart, 1},
    {ExpandEnd, 1},
    // {BreakStart, 1},
    // {BreakEnd, 1},
};

vector<Neighbor> choice_neighbors;

Neighbor neighbor() {
    if (choice_neighbors.empty()) {
        for (auto [neighbor, weight] : neighbor_weights) {
            rep(i, weight) {
                choice_neighbors.push_back(neighbor);
            }
        }
    }
    return choice_neighbors[xorshf96() % choice_neighbors.size()];
}

// 山登り法で最適解を探す
pair<vector<pair<int, int>>, int> hill_climbing(int tl) {
    chrono::system_clock::time_point start = chrono::system_clock::now();
    // まずはランダムなスタート地点を選ぶ
    int r = xorshf96() % n;
    int c = xorshf96() % n;
    // DFSで初期解を求める
    auto [best_moves, best_score] = dfs_greedy(r, c, 1);
    bitset<900> used;
    for (auto [r, c] : best_moves) {
        used[r * n + c] = true;
    }

    // 部分破壊 -> 再構築の操作を繰り返す
    // 焼きなまし
    double start_temp = 1e3;
    double end_temp = 1e-3;
    int updates = 0;
    int iters = 0;
    mt19937 mt(xorshf96());
    while (chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count() < tl) {
        iters++;
        vector<pair<int, int>> new_moves;
        bitset<900> new_used;
        Neighbor nb = neighbor();
        if (nb == Break) {
            // 一部をランダムに破壊
            int moves_size = best_moves.size();
            int l = xorshf96() % (moves_size - 1) + 1;
            int width = xorshf96() % n + 1;
            int r = min(moves_size - 1, l + width);
            auto current_used = used;
            pair<int, int> start_cell = best_moves[l - 1];
            pair<int, int> end_cell = best_moves[r];

            for (int i = l; i <= r; i++) {
                auto [r_, c_] = best_moves[i];
                current_used[r_ * n + c_] = false;
            }

            // startとendの間のマスをdfsで探索し、今よりもさらに長い操作列を求める
            vector<pair<int, int>> founded_moves;
            struct HCDFSNode {
                int r;
                int c;
                vector<pair<int, int>> moves;
                bitset<900> used;
            };
            stack<HCDFSNode> st;
            st.push({start_cell.first, start_cell.second, {start_cell}, current_used});
            // 今の最適解よりも長い操作列が見つかったら、それを最適解とする
            chrono::system_clock::time_point start_dfs = chrono::system_clock::now();
            int tl_dfs = 10;
            while (!st.empty() && chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start_dfs).count() < tl_dfs) {
                auto [cur_r, cur_c, cur_moves, cur_used] = st.top();
                st.pop();
                if (cur_r == end_cell.first && cur_c == end_cell.second) {
                    founded_moves = cur_moves;
                    new_used = cur_used;
                    break;
                }
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
                    auto moves_ = cur_moves;
                    moves_.push_back({r_, c_});
                    nexts.push_back({r_, c_, moves_, used_});
                }

                shuffle(nexts.begin(), nexts.end(), mt);

                for (auto& next : nexts) {
                    st.push(next);
                }
            }

            if (founded_moves.empty()) {
                continue;
            }

            // 最も長い操作列を求めたので、それが既存のパスより長い場合は更新する

            for (int i = 0; i < l - 1; i++) {
                new_moves.push_back(best_moves[i]);
            }
            for (int i = 0; i < founded_moves.size(); i++) {
                new_moves.push_back(founded_moves[i]);
            }
            for (int i = r + 1; i < best_moves.size(); i++) {
                new_moves.push_back(best_moves[i]);
            }
        }

        if (nb == ExpandStart) {
            // スタート地点からrev方面に一マス拡張
            pair<int, int> start_cell = best_moves[0];
            auto [r, c] = start_cell;
            vector<pair<int, int>> candidates;
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
                    if (used[r_ * n + c_]) {
                        continue;
                    }
                    if (revs[arrow] == grid[r_][c_].arrow) {
                        candidates.push_back({r_, c_});
                    }
                }
            }

            if (candidates.empty()) {
                continue;
            }

            pair<int, int> new_start = candidates[xorshf96() % candidates.size()];
            new_moves.push_back(new_start);
            new_moves.insert(new_moves.end(), best_moves.begin(), best_moves.end());
            new_used = used;
            new_used[new_start.first * n + new_start.second] = true;
        }

        if (nb == ExpandEnd) {
            // ゴール地点からrev方面に一マス拡張
            pair<int, int> end_cell = best_moves.back();
            auto [r, c] = end_cell;
            vector<pair<int, int>> candidates;
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
                if (used[r_ * n + c_]) {
                    continue;
                }
                if (revs[arrow] == grid[r_][c_].arrow) {
                    candidates.push_back({r_, c_});
                }
            }

            if (candidates.empty()) {
                continue;
            }

            pair<int, int> new_end = candidates[xorshf96() % candidates.size()];
            new_moves = best_moves;
            new_moves.push_back(new_end);
            new_used = used;
            new_used[new_end.first * n + new_end.second] = true;
        }

        if (nb == BreakStart) {
            for (int i = 1; i < best_moves.size(); i++) {
                new_moves.push_back(best_moves[i]);
            }
            new_used = used;
            new_used[best_moves[0].first * n + best_moves[0].second] = false;
        }

        if (nb == BreakEnd) {
            for (int i = 0; i < best_moves.size() - 1; i++) {
                new_moves.push_back(best_moves[i]);
            }
            new_used = used;
            new_used[best_moves.back().first * n + best_moves.back().second] = false;
        }

        if (new_moves.size() <= 3) {
            continue;
        }
        int new_score = eval(new_moves);

        int d = new_score - best_score;
        double T = start_temp + (end_temp - start_temp) * (chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count()) / tl;

        if (d > 0 || exp(d / T) > (double)(xorshf96() % 1000) / 1000) {
            updates++;
            // eprintln("score:", best_score, "\tdiff:", d, "\tT:", T, "\tprob:", exp(d / T));
            best_moves = new_moves;
            best_score = new_score;
            used = new_used;
        }
    }

    eprintln("updates", updates, "iters", iters);

    return {best_moves, best_score};
}

int main() {
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

    int trial = 10;

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

    rep(i, trial) {
        auto [moves, score] = hill_climbing(all_tl / trial);
        eprintln("trial", i, "score", score);
        if (score > best_score) {
            best_moves = moves;
            best_score = score;
        }
    }

    println(best_moves.size());
    for (auto [r, c] : best_moves) {
        println(r, c);
    }

    // if (validate(best_moves)) {
    //     println("Score =", best_score);
    // } else {
    //     println("Score = -1.0");
    // }

    return 0;
}
