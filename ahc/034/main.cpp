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

const string UP = "U";
const string DOWN = "D";
const string LEFT = "L";
const string RIGHT = "R";
inline string PULL(int amount) { return "+" + to_string(amount); }
inline string RELEASE(int amount) { return "-" + to_string(amount); }

ll simulate(vector<string> ans, vector<vector<ll>> h) {
    ll n = h.size();
    pair<ll, ll> cur = {0, 0};

    ll base = 0;
    rep(i, n) {
        rep(j, n) {
            base += abs(h[i][j]);
        }
    }
    ll diff = 0;

    ll cost = 0;
    ll holding = 0;
    rep(i, ans.size()) {
        // ll tmp_score = 1e9 * ((double)base / (double)(cost + diff));
        // cerr << "SCORE:" << tmp_score << endl;
        // cerr << "base:" << base << " cost:" << cost << " diff:" << diff << " holding:" << holding << endl;
        if (ans[i] == UP) {
            cur.first--;
            cost += 100 + holding;
        } else if (ans[i] == DOWN) {
            cur.first++;
            cost += 100 + holding;
        } else if (ans[i] == LEFT) {
            cur.second--;
            cost += 100 + holding;
        } else if (ans[i] == RIGHT) {
            cur.second++;
            cost += 100 + holding;
        }

        if (ans[i][0] == '+') {
            ll d = stoll(ans[i].substr(1));
            // diff -= 100 * abs(h[cur.first][cur.second]) + 10000;
            h[cur.first][cur.second] -= d;
            holding += d;
            cost += d;
            // diff += 100 * abs(h[cur.first][cur.second]) + 10000;
        } else if (ans[i][0] == '-') {
            ll d = stoll(ans[i].substr(1));
            // diff -= 100 * abs(h[cur.first][cur.second]) + 10000;
            h[cur.first][cur.second] += d;
            holding -= d;
            cost += d;
            // diff += 100 * abs(h[cur.first][cur.second]) + 10000;
        }
    }
    // cerr << "base:" << base << " cost:" << cost << " diff:" << diff << " holding:" << holding << endl;

    ll score = 1e9 * ((double)base / (double)(cost + diff));

    return score;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n;
    input(n);
    vector<vector<ll>> base_h(n, vector<ll>(n));

    // hの平均をとる
    rep(i, n) {
        rep(j, n) {
            input(base_h[i][j]);
        }
    }
    ll avg = 0;

    vector<string> best_ans;
    ll best_score = 0;

    rep(d, 6) {
        vector<vector<ll>> h = base_h;
        pair<ll, ll> cur = {0, 0};
        ll holding = 0;
        set<pair<ll, ll>> visited;
        vector<string> ans;
        ll dir = 0;
        while (visited.size() < n * n) {
            visited.insert(cur);
            if (h[cur.first][cur.second] < avg && holding >= avg - h[cur.first][cur.second]) {
                ans.push_back(RELEASE(avg - h[cur.first][cur.second]));
                holding -= avg - h[cur.first][cur.second];
                h[cur.first][cur.second] = avg;
            }
            if (h[cur.first][cur.second] > avg) {
                ans.push_back(PULL(h[cur.first][cur.second] - avg));
                holding += h[cur.first][cur.second] - avg;
                h[cur.first][cur.second] = avg;
            }

            // 4方向に移動
            if (d == 0) {
                if (cur.first + 1 < n && visited.find({cur.first + 1, cur.second}) == visited.end()) {
                    cur = {cur.first + 1, cur.second};
                    ans.push_back(DOWN);
                    continue;
                }
                if (cur.first - 1 >= 0 && visited.find({cur.first - 1, cur.second}) == visited.end()) {
                    cur = {cur.first - 1, cur.second};
                    ans.push_back(UP);
                    continue;
                }
                if (cur.second + 1 < n && visited.find({cur.first, cur.second + 1}) == visited.end()) {
                    cur = {cur.first, cur.second + 1};
                    ans.push_back(RIGHT);
                    continue;
                }
                if (cur.second - 1 >= 0 && visited.find({cur.first, cur.second - 1}) == visited.end()) {
                    cur = {cur.first, cur.second - 1};
                    ans.push_back(LEFT);
                    continue;
                }
            } else if (d == 1) {
                if (cur.second + 1 < n && visited.find({cur.first, cur.second + 1}) == visited.end()) {
                    cur = {cur.first, cur.second + 1};
                    ans.push_back(RIGHT);
                    continue;
                }
                if (cur.second - 1 >= 0 && visited.find({cur.first, cur.second - 1}) == visited.end()) {
                    cur = {cur.first, cur.second - 1};
                    ans.push_back(LEFT);
                    continue;
                }
                if (cur.first + 1 < n && visited.find({cur.first + 1, cur.second}) == visited.end()) {
                    cur = {cur.first + 1, cur.second};
                    ans.push_back(DOWN);
                    continue;
                }
                if (cur.first - 1 >= 0 && visited.find({cur.first - 1, cur.second}) == visited.end()) {
                    cur = {cur.first - 1, cur.second};
                    ans.push_back(UP);
                    continue;
                }
            } else if (d == 2) {
                if (cur.second + 1 < n && visited.find({cur.first, cur.second + 1}) == visited.end()) {
                    cur = {cur.first, cur.second + 1};
                    ans.push_back(RIGHT);
                    continue;
                }
                if (cur.first + 1 < n && visited.find({cur.first + 1, cur.second}) == visited.end()) {
                    cur = {cur.first + 1, cur.second};
                    ans.push_back(DOWN);
                    continue;
                }
                if (cur.second - 1 >= 0 && visited.find({cur.first, cur.second - 1}) == visited.end()) {
                    cur = {cur.first, cur.second - 1};
                    ans.push_back(LEFT);
                    continue;
                }
                if (cur.first - 1 >= 0 && visited.find({cur.first - 1, cur.second}) == visited.end()) {
                    cur = {cur.first - 1, cur.second};
                    ans.push_back(UP);
                    continue;
                }
            } else if (d == 3) {
                if (cur.first + 1 < n && visited.find({cur.first + 1, cur.second}) == visited.end()) {
                    cur = {cur.first + 1, cur.second};
                    ans.push_back(DOWN);
                    continue;
                }
                if (cur.second + 1 < n && visited.find({cur.first, cur.second + 1}) == visited.end()) {
                    cur = {cur.first, cur.second + 1};
                    ans.push_back(RIGHT);
                    continue;
                }
                if (cur.first - 1 >= 0 && visited.find({cur.first - 1, cur.second}) == visited.end()) {
                    cur = {cur.first - 1, cur.second};
                    ans.push_back(UP);
                    continue;
                }
                if (cur.second - 1 >= 0 && visited.find({cur.first, cur.second - 1}) == visited.end()) {
                    cur = {cur.first, cur.second - 1};
                    ans.push_back(LEFT);
                    continue;
                }
            } else if (d == 4) {
                if (cur.second + 1 < n && visited.find({cur.first, cur.second + 1}) == visited.end() && dir == 0) {
                    if (cur.second + 2 == n || visited.find({cur.first, cur.second + 2}) != visited.end()) {
                        dir = 1;
                    }
                    cur = {cur.first, cur.second + 1};
                    ans.push_back(RIGHT);
                    continue;
                }
                if (cur.first + 1 < n && visited.find({cur.first + 1, cur.second}) == visited.end() && dir == 1) {
                    if (cur.first + 2 == n || visited.find({cur.first + 2, cur.second}) != visited.end()) {
                        dir = 2;
                    }
                    cur = {cur.first + 1, cur.second};
                    ans.push_back(DOWN);
                    continue;
                }
                if (cur.second - 1 >= 0 && visited.find({cur.first, cur.second - 1}) == visited.end() && dir == 2) {
                    if (cur.second - 2 == -1 || visited.find({cur.first, cur.second - 2}) != visited.end()) {
                        dir = 3;
                    }
                    cur = {cur.first, cur.second - 1};
                    ans.push_back(LEFT);
                    continue;
                }
                if (cur.first - 1 >= 0 && visited.find({cur.first - 1, cur.second}) == visited.end() && dir == 3) {
                    if (cur.first - 2 == -1 || visited.find({cur.first - 2, cur.second}) != visited.end()) {
                        dir = 0;
                    }
                    cur = {cur.first - 1, cur.second};
                    ans.push_back(UP);
                    continue;
                }
            } else if (d == 5) {
                if (cur.first + 1 < n && visited.find({cur.first + 1, cur.second}) == visited.end() && dir == 0) {
                    if (cur.first + 2 == n || visited.find({cur.first + 2, cur.second}) != visited.end()) {
                        dir = 1;
                    }
                    cur = {cur.first + 1, cur.second};
                    ans.push_back(DOWN);
                    continue;
                }
                if (cur.second + 1 < n && visited.find({cur.first, cur.second + 1}) == visited.end() && dir == 1) {
                    if (cur.second + 2 == n || visited.find({cur.first, cur.second + 2}) != visited.end()) {
                        dir = 2;
                    }
                    cur = {cur.first, cur.second + 1};
                    ans.push_back(RIGHT);
                    continue;
                }
                if (cur.first - 1 >= 0 && visited.find({cur.first - 1, cur.second}) == visited.end() && dir == 2) {
                    if (cur.first - 2 == -1 || visited.find({cur.first - 2, cur.second}) != visited.end()) {
                        dir = 3;
                    }
                    cur = {cur.first - 1, cur.second};
                    ans.push_back(UP);
                    continue;
                }
                if (cur.second - 1 >= 0 && visited.find({cur.first, cur.second - 1}) == visited.end() && dir == 3) {
                    if (cur.second - 2 == -1 || visited.find({cur.first, cur.second - 2}) != visited.end()) {
                        dir = 0;
                    }
                    cur = {cur.first, cur.second - 1};
                    ans.push_back(LEFT);
                    continue;
                }
            }
        }

        // まだゼロじゃない点を探す
        set<pair<ll, ll>> upper;
        set<pair<ll, ll>> lower;
        rep(i, n) {
            rep(j, n) {
                if (h[i][j] > 0) {
                    upper.insert({i, j});
                }
                if (h[i][j] < 0) {
                    lower.insert({i, j});
                }
            }
        }

        // upperを巡回する
        // for (auto p : upper) {
        while (!upper.empty()) {
            // curから最も近い点を探す
            pair<ll, ll> p = *upper.begin();
            for (auto q : upper) {
                if (abs(cur.first - q.first) + abs(cur.second - q.second) < abs(cur.first - p.first) + abs(cur.second - p.second)) {
                    p = q;
                }
            }
            upper.erase(p);
            while (cur != p) {
                if (cur.first < p.first) {
                    cur = {cur.first + 1, cur.second};
                    ans.push_back(DOWN);
                } else if (cur.first > p.first) {
                    cur = {cur.first - 1, cur.second};
                    ans.push_back(UP);
                } else if (cur.second < p.second) {
                    cur = {cur.first, cur.second + 1};
                    ans.push_back(RIGHT);
                } else {
                    cur = {cur.first, cur.second - 1};
                    ans.push_back(LEFT);
                }
            }
            if (h[cur.first][cur.second] > 0) {
                ans.push_back(PULL(h[cur.first][cur.second] - avg));
                holding += h[cur.first][cur.second];
                h[cur.first][cur.second] = 0;
            }
        }

        // lowerを巡回する
        // for (auto p : lower) {
        while (!lower.empty()) {
            // curから最も近い点を探す
            pair<ll, ll> p = *lower.begin();
            for (auto q : lower) {
                if (abs(cur.first - q.first) + abs(cur.second - q.second) < abs(cur.first - p.first) + abs(cur.second - p.second)) {
                    p = q;
                }
            }
            lower.erase(p);
            while (cur != p) {
                if (cur.first < p.first) {
                    cur = {cur.first + 1, cur.second};
                    ans.push_back(DOWN);
                } else if (cur.first > p.first) {
                    cur = {cur.first - 1, cur.second};
                    ans.push_back(UP);
                } else if (cur.second < p.second) {
                    cur = {cur.first, cur.second + 1};
                    ans.push_back(RIGHT);
                } else {
                    cur = {cur.first, cur.second - 1};
                    ans.push_back(LEFT);
                }
            }
            if (h[cur.first][cur.second] < 0) {
                ans.push_back(RELEASE(avg - h[cur.first][cur.second]));
                holding -= h[cur.first][cur.second];
                h[cur.first][cur.second] = 0;
            }
        }

        ll score = simulate(ans, base_h);
        cerr << "SCORE:" << score << endl;
        if (score > best_score) {
            best_score = score;
            best_ans = ans;
        }
    }

    for (auto s : best_ans) {
        println(s);
    }

    return 0;
}
