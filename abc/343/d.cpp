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

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, t;
    input(n, t);
    vector<ll> a(t), b(t);
    rep(i, t) input(a[i], b[i]);
    vector<ll> s(n, 0);
    map<ll, ll> cnt;
    // 最初0点がn個
    cnt[0] = n;
    rep(i, t) {
        ll prev = s[a[i] - 1];
        s[a[i] - 1] += b[i];
        cnt[prev]--;
        if (cnt[prev] == 0) cnt.erase(prev);
        cnt[s[a[i] - 1]]++;
        println(cnt.size());
    }

    return 0;
}
