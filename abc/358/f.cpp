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

void print_map(map<pair<ll, ll>, pair<ll, ll>> path, ll n, ll m) {
    vector<vector<char>> ans(n * 2 + 1, vector<char>(m * 2 + 1, 'o'));

    for (ll i = 1; i < 2 * n; i += 2) {
        for (ll j = 1; j < 2 * m; j += 2) {
            ans[i + 1][j] = '-';
            ans[i][j + 1] = '|';
        }
    }

    rep(i, 2 * n + 1) {
        ans[i][0] = '+';
        ans[i][2 * m] = '+';
    }
    rep(i, 2 * m + 1) {
        ans[0][i] = '+';
        ans[2 * n][i] = '+';
    }

    for (ll i = 0; i < 2 * n + 1; i++) {
        for (ll j = 0; j < 2 * m + 1; j++) {
            if (i % 2 == 0 && j % 2 == 0) {
                ans[i][j] = '+';
            }
        }
    }

    ans[0][2 * m - 1] = 'S';
    ans[2 * n][2 * m - 1] = 'G';

    auto convert = [&](ll x, ll y) -> pair<ll, ll> {
        return {x * 2 + 1, y * 2 + 1};
    };

    for (auto [k, v] : path) {
        auto [x1, y1] = convert(k.first, k.second);
        auto [x2, y2] = convert(v.first, v.second);
        if (x1 == x2) {
            for (ll i = min(y1, y2) + 1; i < max(y1, y2); i++) {
                ans[x1][i] = '.';
            }
        } else {
            for (ll i = min(x1, x2) + 1; i < max(x1, x2); i++) {
                ans[i][y1] = '.';
            }
        }
    }

    rep(i, 2 * n + 1) {
        rep(j, 2 * m + 1) {
            cout << ans[i][j];
        }
        cout << endl;
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, m, k;
    input(n, m, k);

    if (n % 2 != k % 2) {
        println("No");
        return 0;
    }

    if (n > k) {
        println("No");
        return 0;
    }

    map<pair<ll, ll>, pair<ll, ll>> path;
    ll length = n - 1;
    // 初期解を生成
    rep(i, n - 1) {
        path[{i, m - 1}] = {i + 1, m - 1};
    }
    ll left = k - n;

    ll nv = 0;
    ll nh = 0;

    println("Yes");

    while (nv * 2 + 1 < n && left != 0) {
        ll vx = 2 * nv;
        rep(i, m - 1) {
            if (left == 0) {
                break;
            }
            ll vy = m - 1 - i;
            path.erase({vx, vy});
            path[{vx, vy}] = {vx, vy - 1};
            path[{vx, vy - 1}] = {vx + 1, vy - 1};
            path[{vx + 1, vy - 1}] = {vx + 1, vy};
            left -= 2;
        }
        nv++;
    }

    while (left != 0) {
        ll hx = n - 2;
        ll hy = 2 * nh;
        path.erase({hx, hy});
        path[{hx, hy}] = {hx + 1, hy};
        path[{hx + 1, hy}] = {hx + 1, hy + 1};
        path[{hx + 1, hy + 1}] = {hx, hy + 1};
        left -= 2;

        nh++;
    }

    print_map(path, n, m);

    return 0;
}
