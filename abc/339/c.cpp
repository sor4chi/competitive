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

    ll N;
    input(N);
    vector<ll> A(N);

    ll minimum = 0;
    ll current = 0;
    rep(i, N) {
        input(A[i]);
        if (current + A[i] >= 0) {
            current += A[i];
        } else {
            minimum += abs(current + A[i]);
            current = 0;
        }
    }

    ll sum = 0;
    for (ll a : A) {
        sum += a;
    }
    sum += minimum;
    println(sum);

    return 0;
}
