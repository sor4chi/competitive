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
    vector<vector<ll> > a(3, vector<ll>(3));
    vector<vector<bool> > hitmap(3, vector<bool>(3));
    for (ll i = 0; i < 3; ++i)
        for (ll j = 0; j < 3; ++j) cin >> a[i][j];
    ll n;
    cin >> n;

    for (ll i = 0; i < n; ++i) {
        ll b;
        cin >> b;
        for (ll j = 0; j < 3; ++j)
            for (ll k = 0; k < 3; ++k)
                if (a[j][k] == b) hitmap[j][k] = true;
    }

    // 縦チェック
    for (ll i = 0; i < 3; ++i) {
        bool hit = true;
        for (ll j = 0; j < 3; ++j) {
            if (!hitmap[i][j]) hit = false;
        }
        if (hit) {
            cout << "Yes" << endl;
            return 0;
        }
    }

    // 横チェック
    for (ll i = 0; i < 3; ++i) {
        bool hit = true;
        for (ll j = 0; j < 3; ++j) {
            if (!hitmap[j][i]) hit = false;
        }
        if (hit) {
            cout << "Yes" << endl;
            return 0;
        }
    }

    // 斜めチェック
    if ((hitmap[0][0] && hitmap[1][1] && hitmap[2][2]) ||
        (hitmap[0][2] && hitmap[1][1] && hitmap[2][0])) {
        cout << "Yes" << endl;
        return 0;
    }

    cout << "No" << endl;
    return 0;
}
