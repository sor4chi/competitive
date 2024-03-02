#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
typedef long long ll;
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
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void println() { cout << '\n'; }
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

struct Ad {
    int x, y, r;
};

struct Rect {
    int x1, y1, x2, y2;
    bool operator==(const Rect& r) const {
        return x1 == r.x1 && y1 == r.y1 && x2 == r.x2 && y2 == r.y2;
    }
    bool overlap_with(const Rect& r) const {
        bool x_overlap = (x1 <= r.x1 && r.x1 <= x2) || (x1 <= r.x2 && r.x2 <= x2) || (r.x1 <= x1 && x1 <= r.x2) || (r.x1 <= x2 && x2 <= r.x2);
        bool y_overlap = (y1 <= r.y1 && r.y1 <= y2) || (y1 <= r.y2 && r.y2 <= y2) || (r.y1 <= y1 && y1 <= r.y2) || (r.y1 <= y2 && y2 <= r.y2);
        return x_overlap && y_overlap;
    }
};

int n;
vector<Ad> ads;

void answer(const vector<Rect>& ans) {
    for (auto r : ans) {
        println(r.x1, r.y1, r.x2, r.y2);
    }
}

unsigned long rng() {
    static unsigned long x = 88172645463325252UL;
    x ^= x << 7;
    x ^= x >> 9;
    return x;
}

double rnd() {
    return (double)rng() / (double)ULONG_MAX;
}

random_device seed_gen;
mt19937 engine(seed_gen());

const map<char, pair<ll, ll>> d = {
    {'L', {0, -1}},
    {'R', {0, 1}},
    {'U', {-1, 0}},
    {'D', {1, 0}},
};

const char dir[4] = {'L', 'R', 'U', 'D'};

const int SIZE = 1e4;
double start_temp = 1000;
double end_temp = 10;

void print_report(const vector<Rect>& ans, ll score) {
    cerr << "score: " << score << endl;
    rep(i, n) {
        Rect a = ans[i];
        Ad ad = ads[i];
        int s = (a.x2 - a.x1) * (a.y2 - a.y1);
        cerr << a.x1 << " " << a.y1 << " " << a.x2 << " " << a.y2 << " " << s << endl;
        int diff = s - ad.r;
        cerr << "diff: " << diff << endl;
    }
}

ll evaluate(const vector<Rect>& ans) {
    long double score = 0.0;
    rep(i, n) {
        Ad ad = ads[i];
        Rect a = ans[i];
        double ad_x = ad.x + 0.5;
        double ad_y = ad.y + 0.5;
        if (a.x1 > ad_x || ad_x > a.x2 || a.y1 > ad_y || ad_y > a.y2) {
            score += 0;
        } else {
            int s = (a.x2 - a.x1) * (a.y2 - a.y1);
            double penalty = (1.0 - (double)min(ad.r, s) / (double)max(ad.r, s));
            score += 1.0 - penalty * penalty;
        }
    }
    score /= n;
    score *= 1e9;
    // round to nearest integer
    ll ret = (ll)(score + 0.5);
    return ret;
}

bool validate(const vector<Rect>& ans) {
    vector<vector<bool>> grid(SIZE, vector<bool>(SIZE, false));
    rep(i, n) {
        Rect a = ans[i];
        rep(j, n) {
            if (i == j) continue;
            Rect b = ans[j];
            if (a.overlap_with(b)) return false;
        }
    }
    return true;
}

int indexOf(vector<Rect> v, int data, int idx) {
    auto itr = find_if(v.begin(), v.end(), [&idx, &data](Rect row) {
        if (idx == 0) return row.x1 == data;
        if (idx == 1) return row.y1 == data;
        if (idx == 2) return row.x2 == data;
        if (idx == 3) return row.y2 == data;
    });
    return itr == v.end() ? -1 : distance(v.begin(), itr);
}

vector<double> softmax(vector<int> values) {
    vector<double> ret;
    int sum = 0;
    for (auto v : values) {
        sum += v;
    }
    double minimized_sum = 0.0;
    for (auto v : values) {
        minimized_sum += exp((double)v / (double)sum);
    }
    for (auto v : values) {
        ret.push_back(exp((double)v / (double)sum) / minimized_sum);
    }
    return ret;
}

enum class NeighborType {
    MOVE,
    EXPAND,
    SHRINK,
};

map<NeighborType, double> neighbor_type_probs = {
    {NeighborType::MOVE, 1.0},
    {NeighborType::EXPAND, 1.0},
    {NeighborType::SHRINK, 1.0},
};

NeighborType rand_neighbor_type() {
    double r = rnd();
    double sum = 0;
    for (auto p : neighbor_type_probs) {
        sum += p.second;
    }
    double acc = 0;
    for (auto p : neighbor_type_probs) {
        acc += p.second / sum;
        if (r < acc) {
            return p.first;
        }
    }
    return NeighborType::MOVE;
}

chrono::system_clock::time_point start;

struct Solver {
    vector<Rect> best_ans;
    ll best_score = 0;

    void create_initial(int tl = 1950) {
        // === 初期解生成 ここから ===
        for (auto ad : ads) {
            int x1 = ad.x;
            int y1 = ad.y;
            int x2 = ad.x + 1;
            int y2 = ad.y + 1;
            best_ans.push_back({x1, y1, x2, y2});
        }
        // === 初期解生成 ここまで ===

        // ========== ここから山登り操作 ==========
        chrono::system_clock::time_point time_limit = start + chrono::milliseconds(tl);
        int iter = 0;

        while (chrono::system_clock::now() < time_limit) {
            iter++;

            vector<Rect> ans = best_ans;
            ll score = 0;

            // 要求面積が大きいものが優先的に広げられるようにする
            vector<int> values;
            vector<int> idxs;
            rep(i, n) {
                Rect a = ans[i];
                Ad ad = ads[i];
                int s = (a.x2 - a.x1) * (a.y2 - a.y1);
                if (ad.r > s) {
                    values.push_back(ad.r - s);
                    idxs.push_back(i);
                }
            }
            vector<double> probs = softmax(values);
            vector<pair<int, int>> values_idxs;
            rep(i, values.size()) {
                values_idxs.push_back({values[i], idxs[i]});
            }
            sort(values_idxs.begin(), values_idxs.end(), greater<pair<int, int>>());
            double r = rnd();
            double sum = 0;
            int selected_idx = -1;
            rep(i, values_idxs.size()) {
                sum += probs[i];
                if (sum > r) {
                    selected_idx = values_idxs[i].second;
                    break;
                }
            }
            if (selected_idx == -1) continue;
            int idx = selected_idx;
            Rect a = ans[idx];
            Ad ad = ads[idx];
            int try_left = 100;
            while (try_left--) {
                int dx1 = 0, dx2 = 0, dy1 = 0, dy2 = 0;
                int move_dir = rng() % 4;  // L, R, U, D
                int diff = ad.r - (a.x2 - a.x1) * (a.y2 - a.y1);
                int expand_size = rng() % (int)sqrt(diff) / 15 + 1;
                bool is_in_the_border = false;
                if (move_dir == 0 && a.x1 - expand_size >= 0) {
                    dx1 = -expand_size;
                    is_in_the_border = true;
                }
                if (move_dir == 1 && a.x2 + expand_size < SIZE) {
                    dx2 = expand_size;
                    is_in_the_border = true;
                }
                if (move_dir == 2 && a.y1 - expand_size >= 0) {
                    dy1 = -expand_size;
                    is_in_the_border = true;
                }
                if (move_dir == 3 && a.y2 + expand_size < SIZE) {
                    dy2 = expand_size;
                    is_in_the_border = true;
                }
                if (!is_in_the_border) continue;
                Rect new_a = a;
                new_a.x1 += dx1;
                new_a.x2 += dx2;
                new_a.y1 += dy1;
                new_a.y2 += dy2;
                bool ok = true;
                rep(i, n) {
                    if (i == idx) continue;
                    if (new_a.overlap_with(ans[i])) {
                        ok = false;
                        break;
                    }
                }
                if (!ok) continue;
                ans[idx] = new_a;
                break;
            }

            score = evaluate(ans);

            int diff = score - best_score;

            if (diff > 0) {
                best_score = score;
                best_ans = ans;
            }
        }
        // ========== ここまで山登り操作 ==========
    }

    // 焼きなまし
    void annealing(int tl = 1950) {
        chrono::system_clock::time_point time_limit = start + chrono::milliseconds(tl);
        int iter = 0;
        double temp = start_temp;
        while (chrono::system_clock::now() < time_limit) {
            iter++;
            vector<Rect> ans = best_ans;
            ll score = 0;

            // ここからランダムに操作を行う
            NeighborType nt = rand_neighbor_type();

            if (nt == NeighborType::MOVE) {
                int select_dir = rng() % 4;
                int select_idx = rng() % n;
                Rect a = ans[select_idx];
                int dx = d.at(dir[select_dir]).first;
                int dy = d.at(dir[select_dir]).second;
                Rect new_a = a;
                new_a.x1 += dx;
                new_a.x2 += dx;
                new_a.y1 += dy;
                new_a.y2 += dy;
                if (new_a.x1 < 0 || new_a.x2 >= SIZE || new_a.y1 < 0 || new_a.y2 >= SIZE) continue;
                bool ok = true;
                rep(i, n) {
                    if (i == select_idx) continue;
                    if (new_a.overlap_with(ans[i])) {
                        ok = false;
                        break;
                    }
                }
                if (!ok) continue;
                ans[select_idx] = new_a;
                score = evaluate(ans);
            }
            if (nt == NeighborType::EXPAND) {
                int select_idx = rng() % n;
                Rect a = ans[select_idx];
                Ad ad = ads[select_idx];
                // 4方向で動ける方向に拡張する
                vector<int> dirs = {0, 1, 2, 3};
                shuffle(dirs.begin(), dirs.end(), engine);
                bool is_in_the_border = false;
                int s = (a.x2 - a.x1) * (a.y2 - a.y1);
                if (ad.r <= s) continue;
                for (auto selected_dir : dirs) {
                    int dx1 = 0, dx2 = 0, dy1 = 0, dy2 = 0;
                    int expand_dir = selected_dir;  // L, R, U, D
                    int diff = ads[select_idx].r - (a.x2 - a.x1) * (a.y2 - a.y1);
                    int expand_size = 1;
                    if (expand_dir == 0 && a.x1 - expand_size >= 0) {
                        dx1 = -expand_size;
                        is_in_the_border = true;
                    }
                    if (expand_dir == 1 && a.x2 + expand_size < SIZE) {
                        dx2 = expand_size;
                        is_in_the_border = true;
                    }
                    if (expand_dir == 2 && a.y1 - expand_size >= 0) {
                        dy1 = -expand_size;
                        is_in_the_border = true;
                    }
                    if (expand_dir == 3 && a.y2 + expand_size < SIZE) {
                        dy2 = expand_size;
                        is_in_the_border = true;
                    }
                    if (!is_in_the_border) continue;
                    Rect new_a = a;
                    new_a.x1 += dx1;
                    new_a.x2 += dx2;
                    new_a.y1 += dy1;
                    new_a.y2 += dy2;
                    bool ok = true;
                    rep(i, n) {
                        if (i == select_idx) continue;
                        if (new_a.overlap_with(ans[i])) {
                            ok = false;
                            break;
                        }
                    }
                    if (!ok) continue;
                    ans[select_idx] = new_a;
                    // 広げられたら終わり
                    break;
                }
                score = evaluate(ans);
            }
            if (nt == NeighborType::SHRINK) {
                int select_idx = rng() % n;
                Rect a = ans[select_idx];
                // 4方向で動ける方向に縮める
                vector<int> dirs = {0, 1, 2, 3};
                shuffle(dirs.begin(), dirs.end(), engine);
                bool is_in_the_border = false;
                for (auto selected_dir : dirs) {
                    int dx1 = 0, dx2 = 0, dy1 = 0, dy2 = 0;
                    int shrink_dir = selected_dir;  // L, R, U, D
                    int shrink_size = 1;
                    if (shrink_dir == 0 && a.x1 + shrink_size < a.x2) {
                        dx1 = shrink_size;
                        is_in_the_border = true;
                    }
                    if (shrink_dir == 1 && a.x2 - shrink_size > a.x1) {
                        dx2 = -shrink_size;
                        is_in_the_border = true;
                    }
                    if (shrink_dir == 2 && a.y1 + shrink_size < a.y2) {
                        dy1 = shrink_size;
                        is_in_the_border = true;
                    }
                    if (shrink_dir == 3 && a.y2 - shrink_size > a.y1) {
                        dy2 = -shrink_size;
                        is_in_the_border = true;
                    }
                    if (!is_in_the_border) continue;
                    Rect new_a = a;
                    new_a.x1 += dx1;
                    new_a.x2 += dx2;
                    new_a.y1 += dy1;
                    new_a.y2 += dy2;
                    ans[select_idx] = new_a;
                    // 縮められたら終わり
                    break;
                }
                score = evaluate(ans);
            }

            int diff = score - best_score;

            double r = rnd();
            double prob = exp((double)diff / temp);

            if (diff > 0 || r < prob) {
                best_score = score;
                best_ans = ans;
                // if (!(rng() % 1000)) answer(best_ans);
            }
            temp = start_temp + (end_temp - start_temp) * (double)(chrono::system_clock::now() - start).count() / (double)(time_limit - start).count();
        }
    }
};

int main(int argc, char* argv[]) {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    start = chrono::system_clock::now();

    bool is_arg_contain_export = false;
    for (int i = 0; i < argc; i++) {
        if (string(argv[i]) == "--export") {
            is_arg_contain_export = true;
            break;
        }
    }

    // neighbor_type_probsを正規化
    double sum = 0;
    for (auto p : neighbor_type_probs) {
        sum += p.second;
    }
    for (auto& p : neighbor_type_probs) {
        p.second /= sum;
    }

    input(n);
    rep(i, n) {
        int x, y, r;
        input(x, y, r);
        ads.push_back({x, y, r});
    }

    int initial_tl = n * 5;
    int final_tl = 4950;
    // 抽選
    int election_left = 5;
    ll best_initial_score = 0;
    Solver best_solver;
    while (election_left--) {
        Solver s;
        s.create_initial(initial_tl);
        int score = evaluate(s.best_ans);
        if (score > best_initial_score) {
            best_initial_score = score;
            best_solver = s;
        }
    }
    cerr << "initial score: " << best_initial_score << endl;
    best_solver.annealing(final_tl);
    ll score = evaluate(best_solver.best_ans);
    cerr << "final score: " << score << endl;

    answer(best_solver.best_ans);

    if (is_arg_contain_export) {
        println("Score =", score);
    }

    chrono::system_clock::time_point end = chrono::system_clock::now();
    cerr << "elapsed time: " << chrono::duration_cast<chrono::milliseconds>(end - start).count() << "ms" << endl;

    return 0;
}
