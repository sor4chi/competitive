#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
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

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, k;
    input(n, k);
    vector<ll> r(n);
    rep(i, n) input(r[i]);
    vector<vector<ll>> res;

    function<void(vector<ll>, ll, ll)> dfs = [&](vector<ll> r_cn, ll sum, ll i) {
        if (r_cn.size() == n) {
            if (sum % k == 0) {
                res.push_back(r_cn);
            }
        }

        if (i == n) return;

        for (ll j = 1; j <= r[i]; j++) {
            r_cn.push_back(j);
            dfs(r_cn, sum + j, i + 1);
            r_cn.pop_back();
        }
    };

    dfs({}, 0, 0);

    for (auto x : res) {
        printv(x);
    }

    return 0;
}
