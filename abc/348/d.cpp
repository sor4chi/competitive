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

struct Node {
    ll r, c, e;
};

pair<ll, ll> dir[] = {
    {0, 1},
    {0, -1},
    {1, 0},
    {-1, 0},
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll h, w;
    vector<Node> vec;
    map<pair<ll, ll>, ll> medicine;
    input(h, w);
    pair<ll, ll> S, T;
    vector<vector<char>> s(h, vector<char>(w));
    rep(i, h) {
        rep(j, w) {
            input(s[i][j]);
            if (s[i][j] == 'S') {
                S = {i, j};
            } else if (s[i][j] == 'T') {
                T = {i, j};
            }
        }
    }
    ll n;
    input(n);
    rep(i, n) {
        ll r, c, e;
        input(r, c, e);
        medicine[{--r, --c}] = e;
    }

    if (medicine[S]) {
        vec.push_back({S.first, S.second, medicine[S]});
        medicine[S] = 0;
    }

    while (vec.size()) {
        Node cur = vec.back();
        vec.pop_back();
        queue<pair<ll, ll>> q;
        vector<vector<int>> visited(h, vector<int>(w, -1));
        visited[cur.r][cur.c] = cur.e;
        q.push({cur.r, cur.c});
        while (q.size()) {
            auto [r, c] = q.front();
            q.pop();
            rep(d, 4) {
                ll newr = r + dir[d].first;
                ll newc = c + dir[d].second;
                if (newr < 0 || newr >= h || newc < 0 || newc >= w) continue;
                if (visited[newr][newc] != -1) continue;
                if (s[newr][newc] == '#') continue;
                if (s[newr][newc] == 'T') {
                    println("Yes");
                    return 0;
                }
                visited[newr][newc] = visited[r][c] - 1;
                if (visited[newr][newc]) q.push({newr, newc});
                if (medicine[{newr, newc}]) {
                    vec.push_back({newr, newc, medicine[{newr, newc}]});
                    medicine[{newr, newc}] = 0;
                }
            }
        }
    }

    println("No");

    return 0;
}
