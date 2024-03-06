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

void print_board(vector<vector<int>> a) {
    rep(i, N) {
        rep(j, N) {
            cout << a[i][j];
            if (j < N - 1) cout << ' ';
        }
        cout << endl;
    }
}

random_device seed_gen;
mt19937 engine(seed_gen());

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    init();

    print_board(a);

    chrono::system_clock::time_point start = chrono::system_clock::now();

    int cnt = 1;
    Point first = {0, 0};
    vector<vector<int>> new_a(N, vector<int>(N));
    // bfs
    queue<Point> q;
    q.push(first);
    vector<vector<bool>> visited(N, vector<bool>(N));
    visited[first.x][first.y] = true;
    while (!q.empty()) {
        Point p = q.front();
        q.pop();
        new_a[p.x][p.y] = cnt;
        cnt++;
        for (auto& p2 : E_map[p]) {
            if (!visited[p2.x][p2.y]) {
                visited[p2.x][p2.y] = true;
                q.push(p2);
            }
        }
    }

    print_board(new_a);

    Evaluator ev(new_a);

    double start_temp = 1;
    double end_temp = 1e-9;
    int iter = 0;
    int limit = 60000;
    while (true) {
        chrono::system_clock::time_point now = chrono::system_clock::now();
        double elapsed = chrono::duration_cast<chrono::milliseconds>(now - start).count();
        if (elapsed > limit) break;
        double temp = start_temp + (end_temp - start_temp) * elapsed / limit;
        if (temp < end_temp) break;
        iter++;
        int x1 = rng() % N;
        int y1 = rng() % N;
        int x2 = rng() % N;
        int y2 = rng() % N;
        if (x1 == x2 && y1 == y2) continue;
        int diff = ev.check_swap({x1, y1}, {x2, y2});
        if (iter % 100000 == 0) print_board(ev.a);
        if (diff < 0 || rnd() < exp(-diff / temp)) {
            ev.apply_swap({x1, y1}, {x2, y2});
            string fixed_temp = to_string(temp);
            while (fixed_temp.size() < 10) fixed_temp += ' ';
            cerr << "iter: " << iter << ", temp: " << fixed_temp << ", d: " << ev.d << endl;
        }
    }

    print_board(ev.a);

    return 0;
}
