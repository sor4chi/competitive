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
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void println() { cout << '\n'; }
#define rep(i, n) for (int i = 0; i < n; i++)
#define rep1(i, n) for (int i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

int t, N;
vector<vector<int>> v, h;  // 縦のi - i+1間に壁があるか, 横のi - i+1間に壁があるか, v[N][N-1], h[N-1][N]
vector<vector<int>> a;
int MAX_OPERATIONS;

unsigned long rng() {
    static unsigned long x = 88172645463325252UL;
    x ^= x << 7;
    x ^= x >> 9;
    return x;
}

double rnd() {
    return (double)rng() / ULONG_MAX;
}

struct Point {
    int x, y;
    Point(int x, int y) : x(x), y(y) {
    }
    Point operator+(const Point& p) const {
        return Point(x + p.x, y + p.y);
    }
    void operator+=(const Point& p) {
        x += p.x;
        y += p.y;
    }
    bool operator==(const Point& p) const {
        return x == p.x && y == p.y;
    }
    bool operator!=(const Point& p) const {
        return x != p.x || y != p.y;
    }
    bool operator<(const Point& p) const {
        return x != p.x ? x < p.x : y < p.y;
    }
};

enum Direction { L,
                 R,
                 U,
                 D };

char to_char(Direction t) {
    if (t == L) return 'L';
    if (t == R) return 'R';
    if (t == U) return 'U';
    if (t == D) return 'D';
    return '.';
}

const map<Direction, Point> dirs = {
    {L, {0, -1}},
    {R, {0, 1}},
    {U, {-1, 0}},
    {D, {1, 0}},
};

// 隣接マスの組の集合、壁がある場合は除く
set<set<Point>> E;             // (x1, y1) <-> (x2, y2) 隣接マスの組の集合, setを使ってパスの重複を防いでいる
map<Point, set<Point>> E_map;  // (x, y) -> 隣接マスの組の集合

bool can_move(Point me, Direction dir) {
    if (dir == Direction::L && me.y > 0) {
        return !v[me.x][me.y - 1];
    }
    if (dir == Direction::R && me.y < N - 1) {
        return !v[me.x][me.y];
    }
    if (dir == Direction::U && me.x > 0) {
        return !h[me.x - 1][me.y];
    }
    if (dir == Direction::D && me.x < N - 1) {
        return !h[me.x][me.y];
    }
    return false;
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
        }
    }
    h = vector<vector<int>>(N - 1, vector<int>(N));
    rep(i, N - 1) {
        string s;
        input(s);
        rep(j, N) {
            int t = s[j] - '0';
            h[i][j] = t;
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
            Point p = {i, j};
            set<Point> s;
            for (auto& [dir, diff] : dirs) {
                Point p2 = p + diff;
                if (can_move(p, dir)) {
                    s.insert(p2);
                    E.insert({p, p2});
                }
            }
            E_map[p] = s;
        }
    }
}

struct Evaluator {
    vector<vector<int>> a;
    int d;  // スコア

    Evaluator(vector<vector<int>> a) : a(a) {
        d = calc_all_d(a);
    }

    int calc_around_d(Point p) {
        int res = 0;
        for (auto& p2 : E_map[p]) {
            int diff = a[p.x][p.y] - a[p2.x][p2.y];
            res += diff * diff;
        }
        return res;
    }

    int calc_all_d(vector<vector<int>> a) {
        int res = 0;
        for (auto& e : E) {
            Point p1 = *e.begin();
            Point p2 = *e.rbegin();
            int diff = a[p1.x][p1.y] - a[p2.x][p2.y];
            res += diff * diff;
        }
        return res;
    }

    // p1とp2の値を入れ替え、dを更新してその差分を返す
    int apply_swap(Point p1, Point p2) {
        int before = calc_around_d(p1) + calc_around_d(p2);
        swap(a[p1.x][p1.y], a[p2.x][p2.y]);
        int after = calc_around_d(p1) + calc_around_d(p2);
        d += after - before;
        return after - before;
    }

    // p1とp2の値を入れ替えず、もし入れ替えた場合のdの差分を返す
    int check_swap(Point p1, Point p2) {
        int before = calc_around_d(p1) + calc_around_d(p2);
        swap(a[p1.x][p1.y], a[p2.x][p2.y]);
        int after = calc_around_d(p1) + calc_around_d(p2);
        swap(a[p1.x][p1.y], a[p2.x][p2.y]);
        return after - before;
    }
};

enum OpAfterSwap {
    MOVE_L,
    MOVE_R,
    MOVE_U,
    MOVE_D,
    STAY,
};

const map<OpAfterSwap, Point> after_swap_diffs = {
    {MOVE_L, {0, -1}},
    {MOVE_R, {0, 1}},
    {MOVE_U, {-1, 0}},
    {MOVE_D, {1, 0}},
    {STAY, {0, 0}},
};

char to_char(OpAfterSwap t) {
    if (t == MOVE_L) return 'L';
    if (t == MOVE_R) return 'R';
    if (t == MOVE_U) return 'U';
    if (t == MOVE_D) return 'D';
    return '.';
}

Direction to_dir(OpAfterSwap t) {
    if (t == MOVE_L) return L;
    if (t == MOVE_R) return R;
    if (t == MOVE_U) return U;
    if (t == MOVE_D) return D;
    throw "invalid";
}

enum OpType { SWAP,
              NO_SWAP };

char to_char(OpType t) {
    if (t == SWAP) return '1';
    return '0';
}

struct OperationItem {
    OpType type;      // 交換するかしないか
    OpAfterSwap d_t;  // 高橋の移動先
    OpAfterSwap d_a;  // 青木の移動先

    string to_string() {
        return string(1, to_char(type)) + " " + string(1, to_char(d_t)) + " " + string(1, to_char(d_a));
    }
};

struct Answer {
    Point initial_t, initial_a;
    vector<OperationItem> operations;

    Answer(Point initial_t, Point initial_a, vector<OperationItem> operations) : initial_t(initial_t), initial_a(initial_a), operations(operations) {
    }
};

void answer(Answer ans) {
    println(ans.initial_t.x, ans.initial_t.y, ans.initial_a.x, ans.initial_a.y);
    for (auto& o : ans.operations) {
        println(o.to_string());
    }
}

vector<double> softmax(vector<double> x) {
    double max_x = *max_element(x.begin(), x.end());
    double sum = 0;
    for (auto& xi : x) {
        sum += exp(xi - max_x);
    }
    vector<double> res;
    for (auto& xi : x) {
        res.push_back(exp(xi - max_x) / sum);
    }
    return res;
}

int score(int initial_d, int final_d) {
    return max(1, (int)round(1e6 * log2((double)initial_d / final_d)));
}

vector<Point> get_path(Point from, Point to) {
    // 深さ優先探索でfromからtoまでの最短経路を求める
    // ある点から次に進める点はE_mapによって与えられる
    vector<Point> path;
    map<Point, int> dist;
    stack<Point> st;
    st.push(from);
    dist[from] = 0;
    chrono::system_clock::time_point start = chrono::system_clock::now();
    while (!st.empty() && chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count() < N) {
        Point p = st.top();
        st.pop();
        for (auto& p2 : E_map[p]) {
            if (!dist.count(p2)) {
                dist[p2] = dist[p] + 1;
                st.push(p2);
            }
        }
    }
    Point cur = to;
    while (cur != from) {
        path.push_back(cur);
        for (auto& p2 : E_map[cur]) {
            if (dist.count(p2) && dist[p2] < dist[cur]) {
                cur = p2;
                break;
            }
        }
    }
    path.push_back(from);
    reverse(path.begin(), path.end());
    return path;
}

OpAfterSwap get_op_after_swap_by_movement(Point from, Point to) {
    if (from.x == to.x) {
        if (from.y < to.y) {
            return MOVE_R;
        } else if (from.y > to.y) {
            return MOVE_L;
        }
    } else if (from.y == to.y) {
        if (from.x < to.x) {
            return MOVE_D;
        } else if (from.x > to.x) {
            return MOVE_U;
        }
    }
    return STAY;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);
    chrono::system_clock::time_point start = chrono::system_clock::now();

    init();
    int STACK_DETECTION = N;

    Answer best_ans = Answer({0, 0}, {0, 0}, {});

    // 初期解を作る
    // MAX_OPERATIONS回のランダムな操作をする
    Point initial_t = {rng() % N, rng() % N};
    Point initial_a = {rng() % N, rng() % N};
    Point cur_t = initial_t;
    Point cur_a = initial_a;
    vector<OperationItem> operations;
    Evaluator ev(a);
    int initial_d = ev.d;
    int iter = 0;
    set<Point> visited;
    vector<int> history_d;
    OpType next_op_type = NO_SWAP;
    while (operations.size() < MAX_OPERATIONS) {
        iter++;
        if (next_op_type == SWAP) {
            ev.apply_swap(cur_t, cur_a);
        }
        history_d.push_back(ev.d);
        // もし過去3回分の操作が同じなら脱出するためにvisited以外の点へ移動する
        if (history_d.size() >= STACK_DETECTION) {
            bool is_stacked = true;
            for (int i = 0; i < STACK_DETECTION; i++) {
                if (history_d[history_d.size() - 1 - i] != history_d[history_d.size() - 1 - i - 1]) {
                    is_stacked = false;
                    break;
                }
            }
            if (is_stacked) {
                Point next_t = {rng() % N, rng() % N};
                Point next_a = {rng() % N, rng() % N};
                while (visited.count(next_t) || visited.count(next_a)) {
                    next_t = {rng() % N, rng() % N};
                    next_a = {rng() % N, rng() % N};
                }
                vector<Point> path_t = get_path(cur_t, next_t);
                vector<Point> path_a = get_path(cur_a, next_a);
                // swapせずにnextまで移動する
                for (int i = 0; i < max(path_t.size(), path_a.size()); i++) {
                    Point next_t = i < path_t.size() ? path_t[i] : cur_t;
                    Point next_a = i < path_a.size() ? path_a[i] : cur_a;
                    OpAfterSwap op_t = i < path_t.size() ? get_op_after_swap_by_movement(cur_t, path_t[i]) : STAY;
                    OpAfterSwap op_a = i < path_a.size() ? get_op_after_swap_by_movement(cur_a, path_a[i]) : STAY;
                    if (operations.size() >= MAX_OPERATIONS) break;
                    operations.push_back({NO_SWAP, op_t, op_a});
                    // swapするかどうかを検討したものをvisitedに入れるべきなのでここでは入れない
                    // visited.insert(cur_t);
                    cur_t = next_t;
                    cur_a = next_a;
                }
                // history_dを初期化する
                history_d.clear();
                continue;
            }
        }
        // 高橋と青木のできる操作とその操作をした時のスコア増減
        vector<pair<OperationItem, int>> candidates;
        for (auto& [t_op, t_diff] : after_swap_diffs) {
            for (auto& [a_op, a_diff] : after_swap_diffs) {
                // まずその操作ができるかどうかを確認
                bool can_t = t_op == STAY || can_move(cur_t, to_dir(t_op));  // t_opがSTAYなら常にtrue, そうでなければcan_moveを使って確認
                bool can_a = a_op == STAY || can_move(cur_a, to_dir(a_op));  // a_opがSTAYなら常にtrue, そうでなければcan_moveを使って確認
                if (!can_t || !can_a) continue;
                OperationItem op = {NO_SWAP, t_op, a_op};
                int diff = ev.check_swap(cur_t + t_diff, cur_a + a_diff);
                if (diff < 0) {
                    op.type = SWAP;  // スコアが増えるなら交換する
                }
                candidates.push_back({op, diff});
            }
        }
        vector<double> scores;
        for (auto& [op, diff] : candidates) {
            scores.push_back(op.type == SWAP ? -diff : 1);
        }
        scores = softmax(scores);
        double r = rnd();
        int idx = 0;
        double sum = 0;
        for (int i = 0; i < scores.size(); i++) {
            sum += scores[i];
            if (r < sum) {
                idx = i;
                break;
            }
        }
        auto [op, diff] = candidates[idx];
        operations.push_back({next_op_type, op.d_t, op.d_a});  // 入れ替えだけは前回の操作を使い、動くかどうかは今回の操作を使う
        visited.insert(cur_t);
        next_op_type = op.type;  // 次swapするかどうかを覚えておく
        Point diff_t = after_swap_diffs.at(op.d_t);
        Point diff_a = after_swap_diffs.at(op.d_a);
        cur_t += diff_t;
        cur_a += diff_a;
    }

    cerr << "===== initial solution =====" << endl;

    best_ans = Answer(initial_t, initial_a, operations);

    answer(best_ans);
    cerr << "iter: " << iter << endl;
    cerr << "score_d: " << ev.d << endl;
    cerr << "score: " << score(initial_d, ev.d) << endl;
    cerr << "time: " << chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count() << "ms" << endl;

    return 0;
}
