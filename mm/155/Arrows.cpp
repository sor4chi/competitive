// #pragma GCC target "sse4.2"
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

pair<vector<pair<int, int>>, int> closest_return_greedy(int r, int c) {
    int n = grid.size();
    vector<vector<bool>> used(n, vector<bool>(n));
    vector<pair<int, int>> moves;
    int score = 0;
    int i = 1;
    int r_ = r;
    int c_ = c;
    Arrow dir = Top;
    while (true) {
        if (r_ < 0 || r_ >= n || c_ < 0 || c_ >= n) {
            break;
        }
        if (used[r_][c_]) {
            auto [dr, dc] = dirs[dir];
            r_ += dr;
            c_ += dc;
            continue;
        }

        used[r_][c_] = true;
        moves.push_back({r_, c_});
        score += grid[r_][c_].mult * i;

        auto [arrow, m] = grid[r_][c_];
        dir = arrow;
        i++;
    }

    return {moves, score};
}

struct DFSNode {
    int r;
    int c;
    int i;
    int score;
    short int moves[900];
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
    short int best_moves[900];
    int best_score = 0;
    stack<DFSNode> st;
    short int initial_moves[900];
    rep(i, 900) initial_moves[i] = -1;
    initial_moves[0] = r * n + c;
    DFSNode initial = {r, c, 1, grid[r][c].mult};
    memcpy(initial.moves, initial_moves, sizeof(initial_moves));
    initial.used = used;
    st.push(initial);
    chrono::system_clock::time_point start = chrono::system_clock::now();

    while (!st.empty() && chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count() < tl) {
        auto [r, c, i, score, moves, used] = st.top();
        st.pop();

        if (score > best_score) {
            best_score = score;
            memcpy(best_moves, moves, sizeof(moves));
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
            short int moves_[900];
            memcpy(moves_, moves, sizeof(moves));
            moves_[i] = r_ * n + c_;
            DFSNode next = {r_, c_, i + 1, score + grid[r_][c_].mult * (i + 1)};
            memcpy(next.moves, moves_, sizeof(moves_));
            next.used = used_;
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

    vector<pair<int, int>> best_moves_v;
    rep(i, n * n) {
        if (best_moves[i] == -1) {
            break;
        }
        int r = best_moves[i] / n;
        int c = best_moves[i] % n;
        best_moves_v.push_back({r, c});
    }

    return {best_moves_v, best_score};
}

struct BeamNode {
    int r;
    int c;
    int score;
    vector<pair<int, int>> moves;
    bitset<900> used;
};

pair<vector<pair<int, int>>, int> rev_beam(int beam_width) {
    // gridの5のマスを全て列挙
    vector<tuple<int, int, int>> bigger;
    rep(r, n) {
        rep(c, n) {
            bigger.push_back({grid[r][c].mult, r, c});
        }
    }
    sort(bigger.begin(), bigger.end(), greater<tuple<int, int, int>>());
    vector<BeamNode> beams;
    // 5のマスから最大beam_width個のビームを生成
    rep(i, min(beam_width, (int)bigger.size())) {
        auto [_, r, c] = bigger[i];
        bitset<900> used;
        used[r * n + c] = true;
        beams.push_back({r, c, 0, {{r, c}}, used});
    }
    // 逆向きに探索する
    vector<pair<int, int>> best_moves;
    int best_score = 0;
    int iter = 0;
    chrono::system_clock::time_point start = chrono::system_clock::now();
    while (true) {
        chrono::system_clock::time_point now = chrono::system_clock::now();
        if (chrono::duration_cast<chrono::milliseconds>(now - start).count() > 9000) {
            beam_width = max(beam_width / 2, 10);
        }
        // vector<BeamNode> next_beams;
        priority_queue<BeamNode, vector<BeamNode>, function<bool(const BeamNode&, const BeamNode&)>> next_beams(
            [](const BeamNode& a, const BeamNode& b) {
                return a.score > b.score;
            });
        for (auto& beam : beams) {
            // 逆なので今いる方向から8方向に、自分側を向いている矢印を探す
            for (auto [arrow, dir] : dirs) {
                auto [dr, dc] = dir;
                int r_ = beam.r;
                int c_ = beam.c;
                while (true) {
                    r_ += dr;
                    c_ += dc;
                    if (r_ < 0 || r_ >= n || c_ < 0 || c_ >= n) {
                        break;
                    }
                    if (beam.used[r_ * n + c_]) {
                        continue;
                    }
                    auto [arrow_, m] = grid[r_][c_];
                    if (revs[arrow] == arrow_) {
                        auto used_ = beam.used;
                        used_[r_ * n + c_] = true;
                        auto moves_ = beam.moves;
                        moves_.push_back({r_, c_});
                        BeamNode next = {r_, c_, beam.score + m, moves_, used_};
                        // next_beams.push_back(next);
                        next_beams.push(next);
                        if (next_beams.size() > beam_width) {
                            next_beams.pop();
                        }
                    }
                }
            }
        }

        if (next_beams.empty()) {
            break;
        }

        // sort(next_beams.begin(), next_beams.end(), [](const BeamNode& a, const BeamNode& b) {
        //     return a.score > b.score;
        //     // return (10 * a.moves.size() - a.score) < (10 * b.moves.size() - b.score);
        // });

        beams.clear();

        // // もしiterがmults[5] + mults[3]を超えたら、BEAM_WIDTHを400にする
        // if (iter > mults[5] + mults[3]) {
        //     beam_width = 20;
        // }

        // for (int i = 0; i < min(beam_width, (int)next_beams.size()); i++) {
        //     beams.push_back(next_beams[i]);
        // }

        while (!next_beams.empty()) {
            beams.push_back(next_beams.top());
            next_beams.pop();
        }

        iter++;
    }

    for (auto& beam : beams) {
        int score = 0;
        rep1(i, beam.moves.size()) {
            int rev_i = beam.moves.size() - i;
            auto [r, c] = beam.moves[rev_i];
            auto [a, m] = grid[r][c];
            score += m * i;
        }
        if (score > best_score) {
            eprintln("score:", score);
            best_score = score;
            best_moves = beam.moves;
        }
    }

    reverse(best_moves.begin(), best_moves.end());

    return {best_moves, best_score};
}

map<int, int> N_WIDTH = {
    {8, 30000},
    {9, 18000},
    {10, 12000},
    {11, 8300},
    {12, 6300},
    {13, 4500},
    {14, 3300},
    {15, 2700},
    {16, 2150},
    {17, 1800},
    {18, 1500},
    {19, 1200},
    {20, 950},
    {21, 740},
    {22, 650},
    {23, 530},
    {24, 430},
    {25, 350},
    {26, 300},
    {27, 250},
    {28, 210},
    {29, 190},
    {30, 175},
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

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

    int trial = 1;

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

    if (n <= 13) {
        // 中心から遠く、mが高いマスからスタートする
        sort(start_candidates.begin(), start_candidates.end(), [](const auto& a, const auto& b) {
            // multが高い方が良い
            if (a.mult != b.mult) {
                return a.mult > b.mult;
            }
            // 中心から遠い方が良い
            return a.dist_from_center > b.dist_from_center;
        });

        assert(start_candidates.size() >= trial);

        int beam_width = N_WIDTH[n];
        chrono::system_clock::time_point start = chrono::system_clock::now();
        auto [moves, score] = rev_beam(beam_width);
        chrono::system_clock::time_point end = chrono::system_clock::now();
        eprintln("beam_width:", beam_width, "score:", score, "time:", chrono::duration_cast<chrono::milliseconds>(end - start).count());

        best_moves = moves;
        best_score = score;
    } else {
        // N=8の時は100マス, N=30の時は10となるように滑らかに変化させる
        int N_MIN = 8;
        int N_MAX = 30;
        int TRIAL_MIN = 10;
        int TRIAL_MAX = 100;
        int x = n - N_MIN;
        trial = x * (TRIAL_MIN - TRIAL_MAX) / (N_MAX - N_MIN) + TRIAL_MAX;
        eprintln("trial:", trial);
        trial = min(trial, n * n);
        eprintln("trial(capped):", trial);
        int tl = 9900;
        // 中心に近く、mが低いマスからスタートする
        sort(start_candidates.begin(), start_candidates.end(), [](const auto& a, const auto& b) {
            // multが低い方が良い
            if (a.mult != b.mult) {
                return a.mult < b.mult;
            }
            // 中心に近い方が良い
            return a.dist_from_center < b.dist_from_center;
        });

        assert(start_candidates.size() >= trial);

        rep(i, trial) {
            auto [r, c] = start_candidates[i].pos;
            auto [moves, score] = dfs_greedy(r, c, tl / trial);
            eprintln("start:", r, c, "score:", score);
            if (score > best_score) {
                best_moves = moves;
                best_score = score;
            }
        }
    }

    println(best_moves.size());
    for (auto [r, c] : best_moves) {
        println(r, c);
    }

    println("Score =", best_score);

    return 0;
}
