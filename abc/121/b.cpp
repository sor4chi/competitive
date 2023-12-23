#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
typedef long long ll;
template <class T, class... Ts>
void print(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
    cout << '\n';
}
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void print() { cout << '\n'; }
#define rep(i, n) for (ll i = 0; i < n; i++)

int main() {
    ll n, m, c;
    cin >> n >> m >> c;
    vector<ll> b(m);
    for (ll i = 0; i < m; ++i) cin >> b[i];
    vector<vector<ll> > a(n, vector<ll>(m));
    for (ll i = 0; i < n; ++i) {
        for (ll j = 0; j < m; ++j) cin >> a[i][j];
    }

    ll correct = 0;

    for (ll i = 0; i < n; ++i) {
        ll score = 0;
        for (ll j = 0; j < m; j++) score += b[j] * a[i][j];
        score += c;
        if (score > 0) correct++;
    }

    cout << correct << endl;

    return 0;
}
