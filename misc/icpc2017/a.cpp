#include <bits/stdc++.h>
#if __has_include(<atcoder/all>)
#include <atcoder/all>
using namespace atcoder;
#endif
using namespace std;
using ll = long long;
using ull = unsigned long long;
using P = pair<int, int>;
using vi = vector<int>;
#define rep(i, n) for (int i = 0; i < (n); ++i)

int main() {
    while (1) {
        ll n, m;
        cin >> n >> m;
        if (n == 0 && m == 0) break;
        vector<ll> a(n);
        rep(i, n) cin >> a[i];
        sort(a.rbegin(), a.rend());
        ll maxi = 0;
        rep(i, n) {
            for (ll j = i + 1; j < n; j++) {
                if (a[i] + a[j] <= m) {
                    maxi = max(maxi, a[i] + a[j]);
                }
            }
        }
        if (maxi == 0)
            cout << "NONE" << endl;
        else
            cout << maxi << endl;
    }
    return 0;
}
