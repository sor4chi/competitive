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

ll floor(ll x, ll m) {
    ll r = (x % m + m) % m;
    return (x - r) / m;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, q;
    input(n, q);
    vector<ll> r(n);
    for (ll i = 0; i < n; ++i) {
        cin >> r[i];
    }
    sort(r.begin(), r.end());

    vector<ll> sum(n + 1);
    for (ll i = 0; i < n; ++i) {
        sum[i + 1] = sum[i] + r[i];
    }

    for (ll i = 0; i < q; ++i) {
        ll query;
        cin >> query;

        ll index = upper_bound(sum.begin(), sum.end(), query) - sum.begin();

        print(index - 1);
    }

    return 0;
}
