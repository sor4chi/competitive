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

vector<pair<ll, ll>> dir = {
    {0, 1},
    {1, 0},
    {1, -1},
    {0, -1},
    {-1, 0},
    {-1, 1},
};

struct Node {
    ll x, y, c;
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n;
    input(n);
    vector<pair<ll, ll>> c(n);
    rep(i, n) input(c[i].first, c[i].second);
    pair<ll, ll> cur = {0, 0};
    rep(i, n) {
        pair<ll, ll> from = cur;
        pair<ll, ll> to = c[i];
        set<pair<ll, ll>> visited;
        visited.insert(from);
        stack<Node> st;
        st.push({from.first, from.second, 0});
        while (!st.empty()) {
            Node v = st.top();
            // eprintln("cur", v.x, v.y, v.c);
            if (v.x == to.first && v.y == to.second) {
                println(v.c);
                break;
            }
            st.pop();
            vector<pair<ll, Node>> next;
            for (auto nv : dir) {
                Node v2;
                v2.x = v.x + nv.first;
                v2.y = v.y + nv.second;
                if (visited.count({v2.x, v2.y})) continue;
                v2.c = v.c + 1;
                ll dist = abs(to.first - v2.x) + abs(to.second - v2.y);
                // eprintln("next", v2.x, v2.y, v2.c);
                next.push_back({dist, {v2.x, v2.y, v2.c}});
            }
            sort(next.begin(), next.end(), [](const pair<ll, Node>& a, const pair<ll, Node>& b) {
                if (a.first == b.first) {
                    return a.second.x < b.second.x;
                }
                return a.first > b.first;
            });
            rep(i, next.size()) {
                st.push(next[i].second);
            }
        }

        eprintln("end");
    };

    return 0;
}
