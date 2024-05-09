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
template <class T>
void printv(const T& a, string sep = " ", string end = "\n") {
    for (auto x : a) {
        (void)(cout << x << sep);
    }
    cout << end;
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

    ll n, m;
    input(n, m);
    vector<tuple<ll, ll, ll>> edges;
    rep(i, m) {
        ll k, c;
        input(k, c);
        vector<ll> a(k);
        rep(j, k) {
            input(a[j]);
            a[j]--;
        }

        // そもそも最小全域木はコストが同じ時、全て繋がっていればどの辺でもいいので隣同士繋ぐだけでOK
        rep(j, k - 1) {
            auto u = a[j], v = a[j + 1];
            edges.push_back({c, u, v});
        }
    }

    dsu d(n);
    ll ans = 0;
    // コストが小さい順にソートしないとダメ
    sort(edges.begin(), edges.end());
    for (auto [c, u, v] : edges) {
        if (d.same(u, v)) continue;
        d.merge(u, v);
        ans += c;
    }

    if (d.size(0) != n) {
        println(-1);
        return 0;
    }

    println(ans);

    return 0;
}
