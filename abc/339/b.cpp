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

// define dir
static const vector<pair<ll, ll>> dir = {
    {0, 1},   // right
    {1, 0},   // down
    {0, -1},  // left
    {-1, 0},  // up
};

void print_A(vector<vector<char>>& A, ll H, ll W) {
    rep(i, H) {
        rep(j, W) {
            cout << A[i][j];
        }
        println();
    }
}

// トーラス上の座標を取得する
ll get_torus_h(ll H, ll h) {
    return (h + H) % H;
}

ll get_torus_w(ll W, ll w) {
    return (w + W) % W;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll H, W, N;
    input(H, W, N);

    vector<vector<char>> A(H, vector<char>(W, '.'));
    ll cur_h = 0;
    ll cur_w = 0;
    ll cur_dir = 3;

    rep(i, N) {
        // もし現在いるマスが.なら現在いるマスを#にして90度回転して次のマスに移動する
        if (A[cur_h][cur_w] == '.') {
            A[cur_h][cur_w] = '#';
            cur_dir = (cur_dir + 1) % 4;
            // cur_h += dir[cur_dir].first;
            // cur_w += dir[cur_dir].second;
            cur_h = get_torus_h(H, cur_h + dir[cur_dir].first);
            cur_w = get_torus_w(W, cur_w + dir[cur_dir].second);
        }
        // もし現在いるマスが#なら現在いるマスを.にして-90度回転して次のマスに移動する
        else {
            A[cur_h][cur_w] = '.';
            cur_dir = (cur_dir + 3) % 4;
            // cur_h += dir[cur_dir].first;
            // cur_w += dir[cur_dir].second;
            cur_h = get_torus_h(H, cur_h + dir[cur_dir].first);
            cur_w = get_torus_w(W, cur_w + dir[cur_dir].second);
        }
        // print_A(A, H, W);
        // println("-----");
    }

    // print A
    print_A(A, H, W);

    return 0;
}
