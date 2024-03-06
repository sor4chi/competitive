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

int t, N;
vector<vector<int>> v, h;  // 縦のi - i+1間に壁があるか, 横のi - i+1間に壁があるか, v[N][N-1], h[N-1][N]
vector<vector<int>> a;
// 中心からの距離のマップ、事前にBFSで計算しておくことで経路キャッシュを作っておく
map<pair<int, int>, vector<vector<int>>> dist_maps;
pair<int, int> center;
int walls;
int MAX_OPERATIONS;

enum Direction { U,
                 D,
                 L,
                 R,
                 STAY };

Direction get_direction(pair<int, int> from, pair<int, int> to) {
    if (from.first < to.first) return D;
    if (from.first > to.first) return U;
    if (from.second < to.second) return R;
    if (from.second > to.second) return L;
    return STAY;
}

char to_char(Direction d) {
    if (d == U) return 'U';
    if (d == D) return 'D';
    if (d == L) return 'L';
    if (d == R) return 'R';
    return '.';
}

enum OpType { SWAP,
              NO_SWAP };

char to_char(OpType t) {
    if (t == SWAP) return '1';
    if (t == NO_SWAP) return '0';
    throw;  // unreachable
}

struct OperationItem {
    OpType type;    // 交換するかしないか
    Direction d_t;  // 高橋の移動先
    Direction d_a;  // 青木の移動先
};

struct Answer {
    vector<OperationItem> items;                  // 操作のリスト
    pair<int, int> initial_pos_t, initial_pos_a;  // 高橋の位置, 青木の位置
};

void answer(const Answer& ans) {
    println(ans.initial_pos_t.first, ans.initial_pos_t.second, ans.initial_pos_a.first, ans.initial_pos_a.second);
    for (auto& item : ans.items) {
        println(to_char(item.type), to_char(item.d_t), to_char(item.d_a));
    }
}

struct AroundInfo {
    Direction d;         // そのマスの方向
    pair<int, int> pos;  // そのマスの位置
};

random_device seed_gen;
mt19937 engine(seed_gen());

unsigned long rng() {
    static unsigned long x = 88172645463325252UL;
    x ^= x << 7;
    x ^= x >> 9;
    return x;
}

double rnd() {
    return (double)rng() / ULONG_MAX;
}

vector<vector<double>> generate_goal_heatmap(int N) {
    vector<vector<double>> res(N, vector<double>(N));
    int min_v = 1e9;
    int max_v = -1;
    rep(i, N) {
        rep(j, N) {
            int diff_from_wall = max({i, j, N - 1 - i, N - 1 - j});
            res[i][j] = diff_from_wall;
            if (diff_from_wall < min_v) min_v = diff_from_wall;
            if (diff_from_wall > max_v) max_v = diff_from_wall;
        }
    }
    rep(i, N) {
        rep(j, N) {
            res[i][j] -= min_v;
            res[i][j] /= (max_v - min_v);
            res[i][j] *= N * N;
        }
    }
    return res;
}

vector<double> softmax(vector<double> x) {
    double max_x = *max_element(x.begin(), x.end());
    double sum = 0;
    for (auto& xi : x) {
        xi = exp(xi - max_x);
        sum += xi;
    }
    for (auto& xi : x) {
        xi /= sum;
    }
    return x;
}

int get_index_from_prob(vector<double> prob) {
    double r = rnd();
    double sum = 0;
    rep(i, prob.size()) {
        sum += prob[i];
        if (r < sum) return i;
    }
    return prob.size() - 1;
}

// 隣接マスの組の集合、壁がある場合は除く
set<set<pair<int, int>>> E;

int calc_d(vector<vector<int>>& a) {
    assert(!E.empty());
    // 隣接マスの数字の差の二乗和
    int res = 0;
    for (auto& e : E) {
        int diff = a[e.begin()->first][e.begin()->second] - a[e.rbegin()->first][e.rbegin()->second];
        res += diff * diff;
    }
    return res;
}

bool can_move_to(pair<int, int> pos, Direction d) {
    int i = pos.first, j = pos.second;
    if (d == U) {
        if (i < 1 || i > N - 1) return false;
        if (j < 0 || j > N - 1) return false;
        if (h[i - 1][j] == 1) return false;
    }
    if (d == D) {
        if (i < 0 || i > N - 2) return false;
        if (j < 0 || j > N - 1) return false;
        if (h[i][j] == 1) return false;
    }
    if (d == L) {
        if (i < 0 || i > N - 1) return false;
        if (j < 1 || j > N - 1) return false;
        if (v[i][j - 1] == 1) return false;
    }
    if (d == R) {
        if (i < 0 || i > N - 1) return false;
        if (j < 0 || j > N - 2) return false;
        if (v[i][j] == 1) return false;
    }
    return true;
}

vector<AroundInfo> get_movable_around(pair<int, int> pos, set<pair<int, int>> exclude = {}) {
    vector<AroundInfo> res;
    if (!exclude.count(pos))
        res.push_back({STAY, pos});
    if (can_move_to(pos, U) && !exclude.count({pos.first - 1, pos.second})) {
        res.push_back({U, {pos.first - 1, pos.second}});
    }
    if (can_move_to(pos, D) && !exclude.count({pos.first + 1, pos.second})) {
        res.push_back({D, {pos.first + 1, pos.second}});
    }
    if (can_move_to(pos, L) && !exclude.count({pos.first, pos.second - 1})) {
        res.push_back({L, {pos.first, pos.second - 1}});
    }
    if (can_move_to(pos, R) && !exclude.count({pos.first, pos.second + 1})) {
        res.push_back({R, {pos.first, pos.second + 1}});
    }
    return res;
}

vector<pair<int, int>> get_arounds(pair<int, int> center, int range) {
    vector<pair<int, int>> res;
    for (int i = -range; i <= range; i++) {
        for (int j = -range; j <= range; j++) {
            if (i == 0 && j == 0) continue;
            if (center.first + i < 0 || center.first + i >= N) continue;
            if (center.second + j < 0 || center.second + j >= N) continue;
            res.push_back({center.first + i, center.second + j});
        }
    }
    return res;
}

vector<pair<int, int>> get_closer_arounds(pair<int, int> center, int range) {
    int total = range * range;
    vector<pair<int, int>> res;
    vector<vector<int>> dist_map = dist_maps[center];
    map<int, vector<pair<int, int>>> dist_to_pos;
    rep(i, N) {
        rep(j, N) {
            if (dist_map[i][j]) {
                dist_to_pos[dist_map[i][j]].push_back({i, j});
            }
        }
    }
    int cnt = 0;
    for (int i = 1; i <= N * N; i++) {
        if (dist_to_pos.count(i)) {
            for (auto& pos : dist_to_pos[i]) {
                res.push_back(pos);
                cnt++;
                if (cnt >= total) return res;
            }
        }
    }
}

int eval(vector<vector<int>>& initial_a, vector<vector<int>>& final_a) {
    return max(1, (int)round(1e6 * log2((double)calc_d(initial_a) / calc_d(final_a))));
}

map<pair<int, int>, vector<AroundInfo>> around_cache;

vector<vector<int>> create_dist_map(pair<int, int> first_pos) {
    vector<vector<int>> dist_map = vector<vector<int>>(N, vector<int>(N, -1));
    dist_map[first_pos.first][first_pos.second] = 0;
    queue<pair<int, int>> q;
    q.push(first_pos);
    while (!q.empty()) {
        auto pos = q.front();
        q.pop();
        auto around = get_movable_around(pos);
        for (auto& a : around) {
            if (dist_map[a.pos.first][a.pos.second] == -1) {
                dist_map[a.pos.first][a.pos.second] = dist_map[pos.first][pos.second] + 1;
                q.push(a.pos);
            }
        }
    }
    return dist_map;
}

vector<pair<int, int>> get_path(pair<int, int> start, pair<int, int> goal) {
    vector<vector<int>> dist_map = dist_maps[center];
    vector<pair<int, int>> path_from_start_to_center, path_from_goal_to_center;
    pair<int, int> cur = start;
    while (cur != center) {
        path_from_start_to_center.push_back(cur);
        auto around = get_movable_around(cur);
        vector<pair<int, int>> nexts;
        for (auto& a : around) {
            if (dist_map[a.pos.first][a.pos.second] < dist_map[cur.first][cur.second]) {
                nexts.push_back(a.pos);
            }
        }
        cur = nexts[rng() % nexts.size()];
    }
    cur = goal;
    while (cur != center) {
        path_from_goal_to_center.push_back(cur);
        auto around = get_movable_around(cur);
        vector<pair<int, int>> nexts;
        for (auto& a : around) {
            if (dist_map[a.pos.first][a.pos.second] < dist_map[cur.first][cur.second]) {
                nexts.push_back(a.pos);
            }
        }
        cur = nexts[rng() % nexts.size()];
    }
    pair<int, int> last_common = center;
    while (path_from_start_to_center.size() > 0 && path_from_goal_to_center.size() > 0 && path_from_start_to_center.back() == path_from_goal_to_center.back()) {
        last_common = path_from_start_to_center.back();
        path_from_start_to_center.pop_back();
        path_from_goal_to_center.pop_back();
    }
    reverse(path_from_goal_to_center.begin(), path_from_goal_to_center.end());
    path_from_start_to_center.push_back(last_common);
    path_from_start_to_center.insert(path_from_start_to_center.end(), path_from_goal_to_center.begin(), path_from_goal_to_center.end());
    return path_from_start_to_center;
}

vector<pair<int, int>> get_closest_path(pair<int, int> start, pair<int, int> goal) {
    vector<vector<int>> dist_map = dist_maps[start];
    vector<pair<int, int>> path;
    pair<int, int> cur = goal;
    while (cur != start) {
        path.push_back(cur);
        auto around = get_movable_around(cur);
        vector<pair<int, int>> nexts;
        for (auto& a : around) {
            if (dist_map[a.pos.first][a.pos.second] < dist_map[cur.first][cur.second]) {
                nexts.push_back(a.pos);
            }
        }
        cur = nexts[rng() % nexts.size()];
    }
    path.push_back(start);
    reverse(path.begin(), path.end());
    return path;
}

struct Solver {
    Answer best_ans;
    vector<vector<int>> cur_a;
    int best_score = 1e9;

    Solver() {
        cur_a = a;
    }

    bool should_swap(pair<int, int> pos_t, pair<int, int> pos_a) {
        vector<AroundInfo> around_t = get_movable_around(pos_t, {pos_t});
        vector<AroundInfo> around_a = get_movable_around(pos_a, {pos_a});
        // 交換した方が良いなら交換するようにする
        int t_score_before = 0;
        for (auto& e : around_t) {
            int diff = cur_a[e.pos.first][e.pos.second] - cur_a[pos_t.first][pos_t.second];
            t_score_before += diff * diff;
        }
        int a_score_before = 0;
        for (auto& e : around_a) {
            int diff = cur_a[e.pos.first][e.pos.second] - cur_a[pos_a.first][pos_a.second];
            a_score_before += diff * diff;
        }
        int t_score_after = 0;
        for (auto& e : around_t) {
            int diff = cur_a[e.pos.first][e.pos.second] - cur_a[pos_a.first][pos_a.second];
            t_score_after += diff * diff;
        }
        int a_score_after = 0;
        for (auto& e : around_a) {
            int diff = cur_a[e.pos.first][e.pos.second] - cur_a[pos_t.first][pos_t.second];
            a_score_after += diff * diff;
        }
        return t_score_before + a_score_before > t_score_after + a_score_after;
    }

    int get_diff_if_swap(pair<int, int> pos_t, pair<int, int> pos_a) {
        vector<vector<int>> this_a = cur_a;
        vector<AroundInfo> around_t = get_movable_around(pos_t, {pos_t});
        vector<AroundInfo> around_a = get_movable_around(pos_a, {pos_a});
        // もうd計算しちゃう
        int d_before = calc_d(this_a);
        swap(this_a[pos_t.first][pos_t.second], this_a[pos_a.first][pos_a.second]);
        int d_after = calc_d(this_a);
        return d_before - d_after;
    }

    void solve() {
        pair<int, int> cur_pos_t = {rng() % N, rng() % N};
        pair<int, int> cur_pos_a = {rng() % N, rng() % N};
        best_ans.initial_pos_t = cur_pos_t;
        best_ans.initial_pos_a = cur_pos_a;
        best_ans.items = {};
        queue<pair<int, int>> tq, aq;
        pair<int, int> next_goal_t, next_goal_a;
        chrono::system_clock::time_point start = chrono::system_clock::now();
        bool is_goal_reached = true;
        int turn = 0;
        while (best_ans.items.size() < MAX_OPERATIONS) {
            turn++;
            chrono::system_clock::time_point now = chrono::system_clock::now();
            if (chrono::duration_cast<chrono::milliseconds>(now - start).count() > 1900) break;
            if (tq.empty() && aq.empty() && is_goal_reached) {
                double best_d_diff_by_dist = 0;
                vector<pair<int, int>> arounds_t = N > 20 ? get_arounds(cur_pos_t, 5) : get_closer_arounds(cur_pos_t, 8);
                vector<pair<int, int>> arounds_a = N > 20 ? get_arounds(cur_pos_a, 5) : get_closer_arounds(cur_pos_a, 8);
                for (int i = 0; i < min(arounds_t.size(), arounds_a.size()); i++) {
                    pair<int, int> new_goal_t = arounds_t[i];
                    pair<int, int> new_goal_a = arounds_a[i];
                    int diff = get_diff_if_swap(new_goal_a, new_goal_t);
                    int dist_t = N > 20 ? get_path(cur_pos_t, new_goal_t).size() : get_closest_path(cur_pos_t, new_goal_t).size();
                    int dist_a = N > 20 ? get_path(cur_pos_a, new_goal_a).size() : get_closest_path(cur_pos_a, new_goal_a).size();
                    double d_diff_by_dist = (double)diff / (double)(pow(max(dist_t, dist_a) + 1, 2));
                    if (d_diff_by_dist > best_d_diff_by_dist && rnd() < 0.9) {
                        best_d_diff_by_dist = d_diff_by_dist;
                        next_goal_t = new_goal_t;
                        next_goal_a = new_goal_a;
                    }
                }
                vector<pair<int, int>> path_t = N > 20 ? get_path(cur_pos_t, next_goal_t) : get_closest_path(cur_pos_t, next_goal_t);
                for (auto& e : path_t) tq.push(e);
                vector<pair<int, int>> path_a = N > 20 ? get_path(cur_pos_a, next_goal_a) : get_closest_path(cur_pos_a, next_goal_a);
                for (auto& e : path_a) aq.push(e);
                is_goal_reached = false;
            }

            pair<int, int> next_pos_t = cur_pos_t, next_pos_a = cur_pos_a;
            if (!tq.empty()) {
                next_pos_t = tq.front();
                tq.pop();
            }
            if (!aq.empty()) {
                next_pos_a = aq.front();
                aq.pop();
            }
            Direction d_t = get_direction(cur_pos_t, next_pos_t);
            Direction d_a = get_direction(cur_pos_a, next_pos_a);
            if (next_goal_t == cur_pos_t && next_goal_a == cur_pos_a && !is_goal_reached) {
                swap(cur_a[cur_pos_t.first][cur_pos_t.second], cur_a[cur_pos_a.first][cur_pos_a.second]);
                best_ans.items.push_back({SWAP, d_t, d_a});
                cur_pos_t = next_pos_t;
                cur_pos_a = next_pos_a;
                is_goal_reached = true;
            } else {
                best_ans.items.push_back({NO_SWAP, d_t, d_a});
                cur_pos_t = next_pos_t;
                cur_pos_a = next_pos_a;
            }
            {
                OperationItem latest_op = best_ans.items.back();
                if (latest_op.type == NO_SWAP && latest_op.d_t == STAY && latest_op.d_a == STAY) {
                    best_ans.items.pop_back();
                }
                OperationItem second_latest_op = best_ans.items.size() > 1 ? best_ans.items[best_ans.items.size() - 2] : OperationItem{NO_SWAP, STAY, STAY};
                if (second_latest_op.type == SWAP && second_latest_op.d_t == STAY && second_latest_op.d_a == STAY && latest_op.type == NO_SWAP) {
                    best_ans.items.pop_back();
                    best_ans.items.pop_back();
                    best_ans.items.push_back({SWAP, latest_op.d_t, latest_op.d_a});
                }
                // もし2回連続で同じマスをswapするような操作があったら、それを取り消して無理やり新しいgoalに向かうようにする
                if (second_latest_op.type == SWAP && latest_op.type == SWAP && second_latest_op.d_t == latest_op.d_t && second_latest_op.d_a == latest_op.d_a) {
                    best_ans.items.pop_back();
                    vector<AroundInfo> new_goal_t = get_movable_around(cur_pos_t, {cur_pos_t});
                    vector<AroundInfo> new_goal_a = get_movable_around(cur_pos_a, {cur_pos_a});
                    next_goal_t = new_goal_t[rng() % new_goal_t.size()].pos;
                    next_goal_a = new_goal_a[rng() % new_goal_a.size()].pos;
                    vector<pair<int, int>> path_t = N > 20 ? get_path(cur_pos_t, next_goal_t) : get_closest_path(cur_pos_t, next_goal_t);
                    for (auto& e : path_t) tq.push(e);
                    vector<pair<int, int>> path_a = N > 20 ? get_path(cur_pos_a, next_goal_a) : get_closest_path(cur_pos_a, next_goal_a);
                    for (auto& e : path_a) aq.push(e);
                }
            }
        }
    }
};

void init() {
    input(t, N);
    MAX_OPERATIONS = 4 * N * N;
    v = vector<vector<int>>(N, vector<int>(N - 1));
    rep(i, N) {
        string s;
        input(s);
        rep(j, N - 1) {
            int t = s[j] - '0';
            v[i][j] = t;
            walls += t;
        }
    }
    h = vector<vector<int>>(N - 1, vector<int>(N));
    rep(i, N - 1) {
        string s;
        input(s);
        rep(j, N) {
            int t = s[j] - '0';
            h[i][j] = t;
            walls += t;
        }
    }
    a = vector<vector<int>>(N, vector<int>(N));
    rep(i, N) {
        rep(j, N) {
            int t;
            input(t);
            a[i][j] = t;
        }
    }
    // 隣接マスの組を作っておく
    rep(i, N) {
        rep(j, N) {
            if (j < N - 1 && v[i][j] == 0) {
                set<pair<int, int>> s = {{i, j}, {i, j + 1}};
                E.insert(s);
            }
            if (i < N - 1 && h[i][j] == 0) {
                set<pair<int, int>> s = {{i, j}, {i + 1, j}};
                E.insert(s);
            }
        }
    }
    // 隣接マスの組を作っておく
    center = {N / 2, N / 2};
    if (N > 20) {
        dist_maps[center] = create_dist_map(center);
    } else {
        rep(i, N) {
            rep(j, N) {
                dist_maps[{i, j}] = create_dist_map({i, j});
            }
        }
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    init();

    Solver s;
    s.solve();
    answer(s.best_ans);
    cerr << "calc score: " << eval(a, s.cur_a) << endl;

    return 0;
}
