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

vector<pair<ll, ll>> directions = {
    {1, 0},
    {0, 1},
    {-1, 0},
    {0, -1},
};

int main() {
    while (1) {
        ll n, a, b, d;
        input(n, a, b, d);
        if (n == 0) return 0;
        vector<pair<ll, ll>> obj(n);
        map<ll, vector<ll>> x_obj, y_obj;
        rep(i, n) {
            input(obj[i].first, obj[i].second);
            ll x = obj[i].first;
            ll y = obj[i].second;
            if (!x_obj.count(x)) {
                x_obj[x] = vector<ll>();
            }
            x_obj[x].push_back(y);
            if (!y_obj.count(y)) {
                y_obj[y] = vector<ll>();
            }
            y_obj[y].push_back(x);
        }
        for (auto [k, v] : x_obj) {
            sort(v.begin(), v.end());
            x_obj[k] = v;
        }
        for (auto [k, v] : y_obj) {
            sort(v.begin(), v.end());
            y_obj[k] = v;
        }
        pair<ll, ll> cur = {a, b};
        ll dist = d;
        ll dir = 0;
        map<pair<ll, pair<ll, ll>>, ll> cache;
        while (dist > 0) {
            if (cache.count({dir, cur})) {
                ll prev_dist = cache[{dir, cur}];
                ll loop_span = prev_dist - dist;
                dist %= loop_span;
                if (dist == 0) break;
            }

            pair<ll, ll> d = directions[dir];
            if (d.first == 1) {
                vector<ll> sorted_on_xs = y_obj[cur.second];
                sort(sorted_on_xs.begin(), sorted_on_xs.end());
                ll lower_max = LLONG_MIN;
                rep(i, sorted_on_xs.size()) {
                    if (cur.first < sorted_on_xs[i]) {
                        lower_max = max(sorted_on_xs[i], lower_max);
                    }
                }
                if (lower_max != LONG_MIN) {
                    ll prev_x = cur.first;
                    dir = (dir + 1) % 4;
                    ll to_dist = abs(prev_x - (lower_max - 1));
                    if (to_dist <= dist) {
                        cur.first = lower_max - 1;
                        dist -= to_dist;
                    } else {
                        cur.first += dist;
                        dist = 0;
                    }
                } else {
                    cur.first += dist;
                    dist = 0;
                }
            } else if (d.first == -1) {
                vector<ll> sorted_on_xs = y_obj[cur.second];
                sort(sorted_on_xs.begin(), sorted_on_xs.end(), greater<ll>());
                ll upper_min = LLONG_MAX;
                rep(i, sorted_on_xs.size()) {
                    if (cur.first > sorted_on_xs[i]) {
                        upper_min = min(sorted_on_xs[i], upper_min);
                    }
                }
                if (upper_min != LLONG_MAX) {
                    ll prev_x = cur.first;
                    dir = (dir + 1) % 4;
                    ll to_dist = abs(prev_x - (upper_min + 1));
                    if (to_dist <= dist) {
                        cur.first = upper_min + 1;
                        dist -= to_dist;
                    } else {
                        cur.first -= dist;
                        dist = 0;
                    }
                } else {
                    cur.first -= dist;
                    dist = 0;
                }
            } else if (d.second == 1) {
                vector<ll> sorted_on_ys = x_obj[cur.first];
                sort(sorted_on_ys.begin(), sorted_on_ys.end());
                ll lower_max = LLONG_MIN;
                rep(i, sorted_on_ys.size()) {
                    if (cur.second < sorted_on_ys[i]) {
                        lower_max = max(sorted_on_ys[i], lower_max);
                    }
                }
                if (lower_max != LONG_MIN) {
                    ll prev_y = cur.second;
                    dir = (dir + 1) % 4;
                    ll to_dist = abs(prev_y - (lower_max - 1));
                    // eprintln("to_dist", to_dist);
                    if (to_dist <= dist) {
                        cur.second = lower_max - 1;
                        dist -= to_dist;
                    } else {
                        cur.second += dist;
                        dist = 0;
                    }
                } else {
                    cur.second += dist;
                    dist = 0;
                }
            } else if (d.second == -1) {
                vector<ll> sorted_on_ys = x_obj[cur.first];
                sort(sorted_on_ys.begin(), sorted_on_ys.end(), greater<ll>());
                ll upper_min = LLONG_MAX;
                rep(i, sorted_on_ys.size()) {
                    if (cur.second > sorted_on_ys[i]) {
                        upper_min = min(sorted_on_ys[i], upper_min);
                    }
                }
                if (upper_min != LLONG_MAX) {
                    ll prev_y = cur.second;
                    dir = (dir + 1) % 4;
                    ll to_dist = abs(prev_y - (upper_min + 1));
                    if (to_dist <= dist) {
                        cur.second = upper_min + 1;
                        dist -= to_dist;
                    } else {
                        cur.second -= dist;
                        dist = 0;
                    }
                } else {
                    cur.second -= dist;
                    dist = 0;
                }
            }
            if (!cache.count({dir, cur})) {
                cache[{dir, cur}] = dist;
            }
        }
        println(cur.first, cur.second);
    }

    return 0;
}
