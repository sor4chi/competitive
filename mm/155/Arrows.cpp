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

struct Cell {
    Arrow arrow;
    int mult;
};

pair<vector<pair<int, int>>, int> closest_return_greedy(const vector<vector<Cell>>& grid, int r, int c) {
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

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    int n;
    input(n);
    vector<vector<Cell>> grid(n, vector<Cell>(n));
    rep(r, n) {
        rep(c, n) {
            int a, m;
            input(a, m);
            grid[r][c] = {Arrow(a), m};
        }
    }

    vector<pair<int, int>> best_moves;
    int best_score = 0;

    rep(r, n) {
        rep(c, n) {
            auto [moves, score] = closest_return_greedy(grid, r, c);
            eprintln(r, c, score);
            if (score > best_score) {
                best_score = score;
                best_moves = moves;
            }
        }
    }

    println(best_moves.size());
    for (auto [r, c] : best_moves) {
        println(r, c);
    }

    return 0;
}
