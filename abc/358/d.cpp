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
    vector<ll> a(n), b(m);
    rep(i, n) input(a[i]);
    rep(i, m) input(b[i]);
    sort(b.begin(), b.end());

    priority_queue<pair<ll, ll>, vector<pair<ll, ll>>, greater<pair<ll, ll>>> pq;
    rep(i, n) pq.push({a[i], i});

    ll ans = 0;
    rep(i, m) {
        bool is_ok = false;
        while (!pq.empty()) {
            auto p = pq.top();
            pq.pop();
            if (p.first >= b[i]) {
                ans += p.first;
                is_ok = true;
                break;
            }
        }
        if (!is_ok) {
            println(-1);
            return 0;
        }
    }

    println(ans);

    return 0;
}
