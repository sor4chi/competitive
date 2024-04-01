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
void println() { cout << '\n'; }
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

struct Input {
    ll W, D, N;
    vector<vector<ll>> a;
};

struct Area {
    ll x1, y1, x2, y2;
    bool overlap(Area a) {
        return x1 < a.x2 && x2 > a.x1 && y1 < a.y2 && y2 > a.y1;
    }
    ll width() {
        return x2 - x1;
    }
    ll height() {
        return y2 - y1;
    }
    ll area() {
        return width() * height();
    }
};

unsigned long rng() {
    static unsigned long x = 88172645463325252UL;
    x ^= x << 7;
    x ^= x >> 9;
    return x;
}

int TL = 3000 * 0.95;

enum Mode {
    LeftTopConstruction,
    AggressiveLeftTopConstruction,
    InheritLeftTopConstruction,
    VerticalConstruction,
};

pair<vector<Area>, bool> leftTopConstruction__construct_under(ll x_start, ll y_start, ll start_n, ll d, Input in) {
    cerr << "leftTopConstruction__construct_under(x_start=" << x_start << ", y_start=" << y_start << ", start_n=" << start_n << ", d=" << d << ")" << endl;
    Input input = in;  // copy
    vector<Area> ans_today(input.N);
    reverse(input.a[d].begin(), input.a[d].end());
    bool x_flag = true;
    ll final_n = 0;
    for (ll n = start_n; n < input.N; n++) {
        ll req = input.a[d][n];
        if (x_flag) {
            ll y_left = input.W - y_start;
            ll x_extend = (double)req / y_left + 1;
            if (x_extend > input.W - x_start) break;
            Area a = {x_start, y_start, x_start + x_extend, input.W};
            ans_today[n] = a;
            x_start += x_extend;
        } else {
            ll x_left = input.W - x_start;
            ll y_extend = (double)req / x_left + 1;
            if (y_extend > input.W - y_start) break;
            Area a = {x_start, y_start, input.W, y_start + y_extend};
            ans_today[n] = a;
            y_start += y_extend;
        }
        x_flag = !x_flag;
        final_n = n;
    }
    if (final_n != input.N - 1 && (x_start == input.W)) {
        x_start--;
        ans_today[final_n].x2--;
    }
    if (final_n != input.N - 1 && (y_start == input.W)) {
        y_start--;
        ans_today[final_n].y2--;
    }
    ll left = input.N - final_n - 1;
    // x_startとy_startの小さい方を求める
    ll is_x = x_start < y_start;
    ll left_d = 1;
    for (ll n = final_n + 1; n < input.N; n++) {
        ll start = (is_x ? x_start : y_start);
        if (n == input.N - 1) {
            left_d = input.W - start;
        }
        if (is_x) {
            Area a = {start, y_start, start + left_d, input.W};
            ans_today[n] = a;
            x_start += left_d;
        } else {
            Area a = {x_start, start, input.W, start + left_d};
            ans_today[n] = a;
            y_start += left_d;
        }
    }
    for (ll n = input.N - 1; n >= final_n + 1; n--) {
        Area cur = ans_today[n];
        Area prev = ans_today[n - 1];
        ll cur_req = input.a[d][n];
        if (cur_req < (cur.x2 - cur.x1) * (cur.y2 - cur.y1)) {
            if (is_x) {
                while (true) {
                    ll new_area = (cur.x2 - cur.x1 - 1) * (cur.y2 - cur.y1);
                    if (new_area <= cur_req) break;
                    cur.x1++;
                }
                prev.x2 = cur.x1;
            } else {
                while (true) {
                    ll new_area = (cur.x2 - cur.x1) * (cur.y2 - cur.y1 - 1);
                    if (new_area <= cur_req) break;
                    cur.y1++;
                }
                prev.y2 = cur.y1;
            }
            ans_today[n] = cur;
            ans_today[n - 1] = prev;
        }
    }
    reverse(ans_today.begin(), ans_today.end());
    return {ans_today, x_flag};
}

vector<vector<Area>> leftTopConstruction(Input in) {
    cerr << "mode = leftTopConstruction" << endl;
    Input input = in;  // copy
    vector<vector<Area>> ans(input.D, vector<Area>(input.N));
    for (ll d = 0; d < input.D; d++) {
        auto [ans_today, _] = leftTopConstruction__construct_under(0, 0, 0, d, input);
        ans[d] = ans_today;
    }
    return ans;
}

vector<vector<Area>> aggressiveLeftTopConstruction(Input in) {
    cerr << "mode = aggressiveLeftTopConstruction" << endl;
    Input input = in;  // copy
    // ll max_a_d
    vector<ll> max_a(input.N);
    for (ll n = 0; n < input.N; n++) {
        ll max_a_d = 0;
        for (ll d = 0; d < input.D; d++) {
            max_a_d = max(max_a_d, input.a[d][n]);
        }
        max_a[n] = max_a_d;
    }
    vector<vector<Area>> ans(input.D, vector<Area>(input.N));
    bool is_ans_found = false;
    for (ll n = 1; n <= input.N; n++) {
        vector<vector<Area>> ans_trial(input.D, vector<Area>(input.N));
        for (ll d = 0; d < input.D; d++) {
            // 上から徐々に固定していき、埋められなくなった時点でアウト
            for (ll n2 = 1; n2 < n; n2++) {
                input.a[d][input.N - n2] = max_a[input.N - n2];
            }
            auto [ans_today, _] = leftTopConstruction__construct_under(0, 0, 0, d, input);
            ans_trial[d] = ans_today;
        }
        if (!is_ans_found) {
            is_ans_found = true;
            ans = ans_trial;
        }
        // 要求された面積を全て満たせたらそれを採用
        bool is_satisfiable = true;
        for (ll n = 0; n < input.N; n++) {
            for (ll d = 0; d < input.D; d++) {
                if (input.a[d][n] > (ans_trial[d][n].x2 - ans_trial[d][n].x1) * (ans_trial[d][n].y2 - ans_trial[d][n].y1)) {
                    is_satisfiable = false;
                    break;
                }
            }
        }
        if (is_satisfiable) {
            ans = ans_trial;
        } else {
            break;
        }
    }
    return ans;
}

vector<vector<Area>> inheritLeftTopConstruction(Input in) {
    cerr << "mode = inheritLeftTopConstruction" << endl;
    Input input = in;  // copy
    vector<vector<Area>> ans(input.D, vector<Area>(input.N));
    for (ll d = 0; d < input.D; d++) {
        auto [ans_today, x_flag] = leftTopConstruction__construct_under(0, 0, 0, d, input);
        if (d > 0) {
            vector<Area> ans_yesterday = ans[d - 1];
            set<ll> x_walls;
            set<ll> y_walls;
            for (auto yesterday_rect : ans_yesterday) {
                x_walls.insert(yesterday_rect.x1);
                x_walls.insert(yesterday_rect.x2);
                y_walls.insert(yesterday_rect.y1);
                y_walls.insert(yesterday_rect.y2);
            }
            if (x_flag) {
                x_walls.erase(input.W);
            } else {
                y_walls.erase(input.W);
            }
            for (ll n = 0; n < input.N; n++) {
                // a.x2より大きい最小のx_wallを探す
                ll x_wall = -1;
                ll y_wall = -1;
                for (auto x_wall_candidate : x_walls) {
                    if (x_wall_candidate > ans_today[n].x2) {
                        x_wall = x_wall_candidate;
                    }
                }
                for (auto y_wall_candidate : y_walls) {
                    if (y_wall_candidate > ans_today[n].y2) {
                        y_wall = y_wall_candidate;
                    }
                }
                vector<Area> ans_today_copy = ans_today;
                if (x_wall != -1) {
                    ll before_x2 = ans_today[n].x2;
                    ans_today_copy[n].x2 = x_wall;
                    for (ll n2 = 0; n2 < input.N; n2++) {
                        if (ans_today_copy[n2].x1 == before_x2) {
                            ans_today_copy[n2].x1 = x_wall;
                        }
                        if (ans_today_copy[n2].x2 == before_x2) {
                            ans_today_copy[n2].x2 = x_wall;
                        }
                    }
                }
                if (y_wall != -1) {
                    ll before_y2 = ans_today[n].y2;
                    ans_today_copy[n].y2 = y_wall;
                    for (ll n2 = 0; n2 < input.N; n2++) {
                        if (ans_today_copy[n2].y1 == before_y2) {
                            ans_today_copy[n2].y1 = y_wall;
                        }
                        if (ans_today_copy[n2].y2 == before_y2) {
                            ans_today_copy[n2].y2 = y_wall;
                        }
                    }
                }
                // 要求された面積を全て満たせたらそれを採用
                bool is_satisfiable = true;
                for (ll n = 0; n < input.N; n++) {
                    if (input.a[d][n] > (ans_today_copy[n].x2 - ans_today_copy[n].x1) * (ans_today_copy[n].y2 - ans_today_copy[n].y1)) {
                        is_satisfiable = false;
                        break;
                    }
                }
                if (is_satisfiable) {
                    ans_today = ans_today_copy;
                    if (x_wall != -1) {
                        x_walls.erase(x_wall);
                    }
                    if (y_wall != -1) {
                        y_walls.erase(y_wall);
                    }
                } else {
                    break;
                }
            }
        }
        ans[d] = ans_today;
    }
    return ans;
}

vector<vector<Area>> verticalConstruction(Input in) {
    cerr << "mode = verticalConstruction" << endl;
    Input input = in;  // copy
    vector<vector<Area>> ans(input.D, vector<Area>(input.N));
    for (ll d = 0; d < input.D; d++) {
        vector<Area> ans_today(input.N);
        ll x_start = 0;
        for (ll n = 0; n < input.N; n++) {
            ll req = input.a[d][n];
            for (ll x = x_start; x < input.W; x++) {
                ll w = ((double)req / input.W) + 1;
                ll x2 = x + w;
                if (x2 > input.W) x2 = input.W;
                Area a = {x, 0, x2, input.W};
                ans_today[n] = a;
                x_start = x2;
                break;
            }
        }
        if (d > 0 && rng() % 10) {
            vector<Area> ans_yesterday = ans[d - 1];
            for (ll n = 0; n < input.N; n++) {
                Area a_today_n = ans_today[n];
                Area a_yesterday_n = ans_yesterday[n];
                ll diff = a_yesterday_n.x1 - a_today_n.x1;
                if (diff <= 0) continue;
                vector<Area> ans_today_copy = ans_today;
                for (ll n2 = n; n2 < input.N; n2++) {
                    ans_today_copy[n2].x1 += diff;
                    ans_today_copy[n2].x2 += diff;
                }
                if (ans_today_copy.back().x2 >= input.W) continue;
                ans_today = ans_today_copy;
            }
            for (ll n = 0; n < input.N - 1; n++) {
                Area a_today_n = ans_today[n];
                Area a_yesterday_n = ans_yesterday[n];
                Area a_today_n_next = ans_today[n + 1];
                if (a_today_n.x2 < a_yesterday_n.x1 && a_yesterday_n.x1 < a_today_n_next.x1) {
                    a_today_n.x2 = a_yesterday_n.x1;
                }
                if (a_today_n.x2 < a_yesterday_n.x2 && a_yesterday_n.x2 < a_today_n_next.x1) {
                    a_today_n.x2 = a_yesterday_n.x2;
                }
            }
            if (ans_today.back().x2 < ans_yesterday.back().x2) {
                ans_today.back().x2 = ans_yesterday.back().x2;
            }
            if (ans_today.back().x2 < ans_yesterday.back().x1) {
                ans_today.back().x2 = ans_yesterday.back().x1;
            }
        }
        ans[d] = ans_today;
    }
    return ans;
}

bool is_valid_output(Input input, vector<vector<Area>> ans) {
    ll W = input.W;
    ll D = input.D;
    ll N = input.N;
    for (ll d = 0; d < D; d++) {
        vector<vector<bool>> used(W, vector<bool>(W, false));
        for (ll n = 0; n < N; n++) {
            Area a = ans[d][n];
            if (a.x1 < 0 || a.y1 < 0 || a.x2 > W || a.y2 > W) {
                return false;
            }
            if (a.x1 >= a.x2 || a.y1 >= a.y2) {
                return false;
            }
            if (a.x2 - a.x1 > W || a.y2 - a.y1 > W) {
                return false;
            }
            if (a.x2 - a.x1 <= 0 || a.y2 - a.y1 <= 0) {  // w/hが0だとRectにならないのでだめ
                return false;
            }
            for (ll x = a.x1; x < a.x2; x++) {
                for (ll y = a.y1; y < a.y2; y++) {
                    if (used[x][y]) {
                        return false;
                    }
                    used[x][y] = true;
                }
            }
        }
    }
    return true;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    Input input;
    cin >> input.W >> input.D >> input.N;
    input.a = vector<vector<ll>>(input.D, vector<ll>(input.N));
    for (ll d = 0; d < input.D; d++) {
        for (ll n = 0; n < input.N; n++) {
            cin >> input.a[d][n];
        }
    }

    Mode mode = InheritLeftTopConstruction;

    vector<vector<Area>> ans(input.D, vector<Area>(input.N));

    if (mode == LeftTopConstruction) {
        ans = leftTopConstruction(input);
    }
    if (mode == AggressiveLeftTopConstruction) {
        ans = aggressiveLeftTopConstruction(input);
    }
    if (mode == InheritLeftTopConstruction) {
        ans = inheritLeftTopConstruction(input);
    }
    bool validate = is_valid_output(input, ans);
    if (!validate) {
        cerr << "invalid output detected, fallback to verticalConstruction" << endl;
        mode = VerticalConstruction;
    } else {
        cerr << "success output" << endl;
    }
    if (mode == VerticalConstruction) {
        ans = verticalConstruction(input);
    }

    // print ans
    for (ll d = 0; d < input.D; d++) {
        for (ll n = 0; n < input.N; n++) {
            print(ans[d][n].x1, ans[d][n].y1, ans[d][n].x2, ans[d][n].y2);
            cout << endl;
        }
    }

    return 0;
}
