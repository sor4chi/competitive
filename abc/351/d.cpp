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

const vector<pair<ll, ll>> dir = {
    {0, 1},
    {0, -1},
    {1, 0},
    {-1, 0},
};

void print_map(const vector<string>& s) {
    for (auto row : s) {
        println(row);
    }
    println();
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll h, w;
    input(h, w);
    vector<string> s(h);
    rep(i, h) cin >> s[i];

    ll max_size = 0;
    set<pair<ll, ll>> watched;
    rep(i, h) rep(j, w) {
        set<pair<ll, ll>> visited;
        if (s[i][j] == '#') continue;
        if (watched.count({i, j})) continue;
        // 四隣に壁があるかどうかを確認する
        bool is_wall_exist_in_around = false;
        for (auto [dx, dy] : dir) {
            ll nx = i + dx;
            ll ny = j + dy;
            if (nx < 0 || nx >= h || ny < 0 || ny >= w) continue;
            if (s[nx][ny] == '#') {
                is_wall_exist_in_around = true;
                break;
            }
        }
        if (is_wall_exist_in_around) {
            max_size = max(max_size, 1LL);
            continue;
        }
        stack<pair<ll, ll>> st;
        st.push({i, j});
        visited.insert({i, j});
        watched.insert({i, j});
        while (!st.empty()) {
            auto [x, y] = st.top();
            st.pop();
            vector<pair<ll, ll>> next;
            bool is_wall_exist_in_around = false;
            for (auto [dx, dy] : dir) {
                ll nx = x + dx;
                ll ny = y + dy;
                if (nx < 0 || nx >= h || ny < 0 || ny >= w) continue;
                if (s[nx][ny] == '#') {
                    is_wall_exist_in_around = true;
                    continue;
                }
                if (visited.count({nx, ny})) continue;
                next.push_back({nx, ny});
            }
            if (is_wall_exist_in_around) continue;
            for (auto [nx, ny] : next) {
                st.push({nx, ny});
                visited.insert({nx, ny});
                watched.insert({nx, ny});
            }
        }

        max_size = max(max_size, (ll)visited.size());
    }

    println(max_size);

    return 0;
}
