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

vector<vector<char>> sponge;
char VOID = '.';
char WALL = '#';

void generateCarpet(ll level, ll x, ll y, ll size) {
    if (level == 0) {
        sponge[x][y] = WALL;
        return;
    }

    ll shrinkSize = size / 3;
    rep(i, 3) {
        rep(j, 3) {
            if (i == 1 && j == 1) continue;
            generateCarpet(level - 1, x + i * shrinkSize, y + j * shrinkSize, shrinkSize);
        }
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n;
    input(n);

    sponge.resize(pow(3, n), vector<char>(pow(3, n), VOID));

    ll size = pow(3, n);

    generateCarpet(n, 0, 0, size);

    rep(i, size) {
        rep(j, size) {
            cout << sponge[i][j];
        }
        cout << '\n';
    }

    return 0;
}
