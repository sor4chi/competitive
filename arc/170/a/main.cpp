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

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll N;
    string S, T;
    input(N, S, T);

    if ((S[0] == 'A' && T[0] == 'B') || (S[N - 1] == 'B' && T[N - 1] == 'A')) {
        println(-1);
        return 0;
    }

    vector<ll> diff(N, 0);
    rep(i, N) {
        if (S[i] == 'A' && T[i] == 'B') {
            diff[i] = 1;
        } else if (S[i] == 'B' && T[i] == 'A') {
            diff[i] = -1;
        }
    }

    ll ans = 0;
    ll cnt = 0;
    rep(i, N) {
        if (diff[i] == 1) {
            cnt++;
        } else if (diff[i] == -1) {
            cnt--;
        }
        if (cnt > 0) {
            ans++;
            cnt--;
        }
    }
    ans += cnt / 2;

    println(ans);

    return 0;
}
