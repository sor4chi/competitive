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

    ll n, k;
    input(n, k);
    vector<ll> a(k);
    for (ll& i : a) cin >> i;

    vector<ll> presum(k + 1), sufsum(k + 1);
    for (int i = 1; i <= k; i++) {
        presum[i] = presum[i - 1];
        if (i % 2 == 0) presum[i] += a[i - 1] - a[i - 2];
    }
    for (int i = k - 1; i >= 0; i--) {
        sufsum[i] = sufsum[i + 1];
        if ((k - i) % 2 == 0) sufsum[i] += a[i + 1] - a[i];
    }

    ll ans = 1LL << 60;
    for (int i = 0; i <= k; i += 2) {
        ans = min(ans, presum[i] + sufsum[i]);
    }

    print(ans);

    return 0;
}
