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

const map<char, pair<ll, ll>> d = {
    {'U', {-1, 0}},
    {'D', {1, 0}},
    {'L', {0, -1}},
    {'R', {0, 1}},
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    int H, W, N;
    input(H, W, N);
    string T;
    input(T);             // UDLR
    vector<string> S(H);  // . is path, # is wall
    rep(i, H) input(S[i]);

    // in each block, the number of steps satisfies T moves
    ll success_count = 0;
    rep(i, H) {
        rep(j, W) {
            if (S[i][j] == '#') {
                continue;
            }
            // i, j is the start point
            ll y = i, x = j;
            bool is_failed = false;
            for (char t : T) {
                ll dy, dx;
                tie(dy, dx) = d.at(t);
                ll ny = y + dy, nx = x + dx;
                if (ny < 0 || ny >= H || nx < 0 || nx >= W || S[ny][nx] == '#') {
                    is_failed = true;
                    break;
                }
                y = ny;
                x = nx;
            }
            if (!is_failed) {
                success_count++;
            }
        }
    }

    println(success_count);

    return 0;
}
