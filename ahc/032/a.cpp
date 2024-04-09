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

struct Input {
    ll n, m, k;
    vector<vector<ll>> a;
    vector<vector<vector<ll>>> stamps;
};

struct Evaluator {
    vector<vector<vector<ll>>> mod_a_history;
    vector<vector<ll>> mod_a;
    vector<ll> sum_history;
    ll sum;

    Evaluator(vector<vector<ll>> a) {
        mod_a.resize(a.size(), vector<ll>(a.size()));
        sum = calc_all_score(a);
        mod_a_history.push_back(mod_a);
        sum_history.push_back(sum);
    }

    ll calc_all_score(vector<vector<ll>> a) {
        ll res = 0;
        rep(i, a.size()) rep(j, a.size()) {
            ll score = a[i][j];
            int mod_score = score % 998244353;
            res += mod_score;
            mod_a[i][j] = mod_score;
        }
        return res;
    }

    // stampを(x, y)に置いたときのスコアの変化を計算する
    ll check_if_apply_stamp(vector<vector<ll>> stamp, ll x, ll y) {
        ll before_partial_sum = 0;
        rep(i, 3) rep(j, 3) {
            before_partial_sum += mod_a[x + i][y + j];
        }
        ll after_partial_sum = 0;
        rep(i, 3) rep(j, 3) {
            after_partial_sum += (mod_a[x + i][y + j] + stamp[i][j]) % 998244353;
        }
        return sum + after_partial_sum - before_partial_sum;
    }

    void apply_stamp(vector<vector<ll>> stamp, ll x, ll y) {
        sum = check_if_apply_stamp(stamp, x, y);
        rep(i, 3) rep(j, 3) {
            mod_a[x + i][y + j] = (mod_a[x + i][y + j] + stamp[i][j]) % 998244353;
        }
        mod_a_history.push_back(mod_a);
        sum_history.push_back(sum);
    }

    void undo(int count = 1) {
        rep(i, count) {
            mod_a_history.pop_back();
            sum_history.pop_back();
        }
        mod_a = mod_a_history.back();
        sum = sum_history.back();
    }
};

struct Step {
    ll stamp_id, x, y;
};

unsigned long rng() {
    static unsigned long x = 88172645463325252UL;
    x ^= x << 7;
    x ^= x >> 9;
    return x;
}

double rng_double() {
    return rng() / (ULONG_MAX + 1.0);
}

// eg: generate_combinations(3, 2) -> [[0], [1], [2], [0, 0], [0, 1], [0, 2], [1, 1], [1, 2], [2, 0], [2, 1], [2, 2]]
vector<vector<ll>> generate_combinations(int n, int r) {
    vector<vector<ll>> res;
    function<void(vector<ll>, int)> dfs = [&](vector<ll> cur, int depth) {
        if (depth == r) {
            res.push_back(cur);
            return;
        }
        rep(i, n) {
            vector<ll> next = cur;
            next.push_back(i);
            dfs(next, depth + 1);
        }
    };
    dfs({}, 0);
    return res;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    Input in;
    input(in.n, in.m, in.k);
    in.a.resize(in.n, vector<ll>(in.n));
    rep(i, in.n) rep(j, in.n) input(in.a[i][j]);
    in.stamps.resize(in.m);
    rep(i, in.m) {
        vector<vector<ll>> s(3, vector<ll>(3));
        rep(i, 3) rep(j, 3) input(s[i][j]);
        in.stamps[i] = s;
    }

    vector<Step> final_ans;
    vector<vector<ll>> final_a(in.n, vector<ll>(in.n));
    ll final_i = -1;
    ll final_j = -1;
    ll final_score = -1;

    bool enable_lt = true;
    bool enable_lb = true;
    bool enable_rt = true;
    bool enable_rb = true;

    if (enable_lt) {
        // 左上から順にスタンプを置いていく
        vector<Step> best_ans;
        vector<vector<ll>> a(in.n, vector<ll>(in.n));
        rep(i, in.n) rep(j, in.n) a[i][j] = in.a[i][j];
        Evaluator ev(a);
        rep(i, in.n - 3) {
            rep(j, in.n - 3) {
                vector<vector<ll>> a_part(3, vector<ll>(3));
                rep(ii, 3) rep(jj, 3) {
                    a_part[ii][jj] = a[i + ii][j + jj];
                }
                ll best_stamp_id = -1;
                ll best_mod_a = -1;
                vector<vector<ll>> best_a_part;
                rep(k, in.m) {
                    vector<vector<ll>> cur_a_part = a_part;
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[k][ii][jj]) % 998244353;
                    }
                    if (cur_a_part[0][0] > best_mod_a) {
                        best_mod_a = cur_a_part[0][0];
                        best_stamp_id = k;
                        best_a_part = cur_a_part;
                    }
                }
                best_ans.push_back({best_stamp_id, i, j});
                ev.apply_stamp(in.stamps[best_stamp_id], i, j);
                rep(ii, 3) rep(jj, 3) {
                    a[i + ii][j + jj] = best_a_part[ii][jj];
                }
            }
        }

        // 次は右端の列
        rep(i, in.n - 3) {
            int j = in.n - 3;
            vector<vector<ll>> a_part(3, vector<ll>(3));
            rep(ii, 3) rep(jj, 3) {
                a_part[ii][jj] = a[i + ii][j + jj];
            }
            vector<ll> best_stamp_ids;
            ll best_mod_a_sum = -1;
            vector<vector<ll>> best_a_part;
            vector<vector<ll>> stamp_ids_candidates;
            stamp_ids_candidates = generate_combinations(in.m, 3);
            for (auto stamp_ids : stamp_ids_candidates) {
                vector<vector<ll>> cur_a_part = a_part;
                for (auto stamp_id : stamp_ids) {
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                    }
                }
                ll sum = 0;
                rep(jj, 3) {
                    sum += cur_a_part[0][jj];
                }
                if (sum > best_mod_a_sum) {
                    best_mod_a_sum = sum;
                    best_stamp_ids = stamp_ids;
                    best_a_part = cur_a_part;
                }
            }
            for (auto best_stamp_id : best_stamp_ids) {
                best_ans.push_back({best_stamp_id, i, j});
                ev.apply_stamp(in.stamps[best_stamp_id], i, j);
            }
            rep(ii, 3) rep(jj, 3) {
                a[i + ii][j + jj] = best_a_part[ii][jj];
            }
        }

        // 次は下端の行
        rep(j, in.n - 3) {
            int i = in.n - 3;
            vector<vector<ll>> a_part(3, vector<ll>(3));
            rep(ii, 3) rep(jj, 3) {
                a_part[ii][jj] = a[i + ii][j + jj];
            }
            vector<ll> best_stamp_ids;
            ll best_mod_a_sum = -1;
            vector<vector<ll>> best_a_part;
            vector<vector<ll>> stamp_ids_candidates;
            stamp_ids_candidates = generate_combinations(in.m, 3);
            for (auto stamp_ids : stamp_ids_candidates) {
                vector<vector<ll>> cur_a_part = a_part;
                for (auto stamp_id : stamp_ids) {
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                    }
                }
                ll sum = 0;
                rep(ii, 3) {
                    sum += cur_a_part[ii][0];
                }
                if (sum > best_mod_a_sum) {
                    best_mod_a_sum = sum;
                    best_stamp_ids = stamp_ids;
                    best_a_part = cur_a_part;
                }
            }
            for (auto best_stamp_id : best_stamp_ids) {
                best_ans.push_back({best_stamp_id, i, j});
                ev.apply_stamp(in.stamps[best_stamp_id], i, j);
            }
            rep(ii, 3) rep(jj, 3) {
                a[i + ii][j + jj] = best_a_part[ii][jj];
            }
        }

        int i = in.n - 3;
        int j = in.n - 3;
        vector<vector<ll>> a_part(3, vector<ll>(3));
        rep(ii, 3) rep(jj, 3) {
            a_part[ii][jj] = a[i + ii][j + jj];
        }
        vector<ll> best_stamp_ids;
        ll best_mod_a_sum = -1;
        vector<vector<ll>> best_a_part;
        vector<vector<ll>> stamp_ids_candidates;
        stamp_ids_candidates = generate_combinations(in.m, 4);
        for (auto stamp_ids : stamp_ids_candidates) {
            vector<vector<ll>> cur_a_part = a_part;
            for (auto stamp_id : stamp_ids) {
                rep(ii, 3) rep(jj, 3) {
                    cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                }
            }
            ll sum = 0;
            rep(ii, 3) rep(jj, 3) {
                sum += cur_a_part[ii][jj];
            }
            if (sum > best_mod_a_sum) {
                best_mod_a_sum = sum;
                best_stamp_ids = stamp_ids;
                best_a_part = cur_a_part;
            }
        }
        for (auto best_stamp_id : best_stamp_ids) {
            ev.apply_stamp(in.stamps[best_stamp_id], i, j);
        }

        cerr << "left top: " << ev.sum << endl;

        if (ev.sum > final_score) {
            final_ans = best_ans;
            final_score = ev.sum;
            final_a = a;
            final_i = i;
            final_j = j;
        }
    }

    if (enable_lb) {
        // 左下から順にスタンプを置いていく
        vector<Step> best_ans;
        vector<vector<ll>> a(in.n, vector<ll>(in.n));
        rep(i, in.n) rep(j, in.n) a[i][j] = in.a[i][j];
        Evaluator ev(a);
        rep(i, in.n - 3) {
            int i_rev = in.n - 3 - i;
            rep(j, in.n - 3) {
                vector<vector<ll>> a_part(3, vector<ll>(3));
                rep(ii, 3) rep(jj, 3) {
                    a_part[ii][jj] = a[i_rev + ii][j + jj];
                }
                ll best_stamp_id = -1;
                ll best_mod_a = -1;
                vector<vector<ll>> best_a_part;
                rep(k, in.m) {
                    vector<vector<ll>> cur_a_part = a_part;
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[k][ii][jj]) % 998244353;
                    }
                    if (cur_a_part[2][0] > best_mod_a) {
                        best_mod_a = cur_a_part[2][0];
                        best_stamp_id = k;
                        best_a_part = cur_a_part;
                    }
                }
                best_ans.push_back({best_stamp_id, i_rev, j});
                ev.apply_stamp(in.stamps[best_stamp_id], i_rev, j);
                rep(ii, 3) rep(jj, 3) {
                    a[i_rev + ii][j + jj] = best_a_part[ii][jj];
                }
            }
        }

        // 次は右端の列
        rep(i, in.n - 3) {
            int i_rev = in.n - 3 - i;
            int j = in.n - 3;
            vector<vector<ll>> a_part(3, vector<ll>(3));
            rep(ii, 3) rep(jj, 3) {
                a_part[ii][jj] = a[i_rev + ii][j + jj];
            }
            vector<ll> best_stamp_ids;
            ll best_mod_a_sum = -1;
            vector<vector<ll>> best_a_part;
            vector<vector<ll>> stamp_ids_candidates;
            stamp_ids_candidates = generate_combinations(in.m, 3);
            for (auto stamp_ids : stamp_ids_candidates) {
                vector<vector<ll>> cur_a_part = a_part;
                for (auto stamp_id : stamp_ids) {
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                    }
                }
                ll sum = 0;
                rep(jj, 3) {
                    sum += cur_a_part[2][jj];
                }
                if (sum > best_mod_a_sum) {
                    best_mod_a_sum = sum;
                    best_stamp_ids = stamp_ids;
                    best_a_part = cur_a_part;
                }
            }
            for (auto best_stamp_id : best_stamp_ids) {
                best_ans.push_back({best_stamp_id, i_rev, j});
                ev.apply_stamp(in.stamps[best_stamp_id], i_rev, j);
            }
            rep(ii, 3) rep(jj, 3) {
                a[i_rev + ii][j + jj] = best_a_part[ii][jj];
            }
        }

        // 次は上端の行
        rep(j, in.n - 3) {
            int i = in.n - 3;
            int i_rev = in.n - 3 - i;
            vector<vector<ll>> a_part(3, vector<ll>(3));
            rep(ii, 3) rep(jj, 3) {
                a_part[ii][jj] = a[i_rev + ii][j + jj];
            }
            vector<ll> best_stamp_ids;
            ll best_mod_a_sum = -1;
            vector<vector<ll>> best_a_part;
            vector<vector<ll>> stamp_ids_candidates;
            stamp_ids_candidates = generate_combinations(in.m, 3);
            for (auto stamp_ids : stamp_ids_candidates) {
                vector<vector<ll>> cur_a_part = a_part;
                for (auto stamp_id : stamp_ids) {
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                    }
                }
                ll sum = 0;
                rep(ii, 3) {
                    sum += cur_a_part[ii][0];
                }
                if (sum > best_mod_a_sum) {
                    best_mod_a_sum = sum;
                    best_stamp_ids = stamp_ids;
                    best_a_part = cur_a_part;
                }
            }
            for (auto best_stamp_id : best_stamp_ids) {
                best_ans.push_back({best_stamp_id, i_rev, j});
                ev.apply_stamp(in.stamps[best_stamp_id], i_rev, j);
            }
            rep(ii, 3) rep(jj, 3) {
                a[i_rev + ii][j + jj] = best_a_part[ii][jj];
            }
        }

        int i = in.n - 3;
        int i_rev = in.n - 3 - i;
        int j = in.n - 3;
        vector<vector<ll>> a_part(3, vector<ll>(3));
        rep(ii, 3) rep(jj, 3) {
            a_part[ii][jj] = a[i_rev + ii][j + jj];
        }
        vector<ll> best_stamp_ids;
        ll best_mod_a_sum = -1;
        vector<vector<ll>> best_a_part;
        vector<vector<ll>> stamp_ids_candidates;
        stamp_ids_candidates = generate_combinations(in.m, 4);
        for (auto stamp_ids : stamp_ids_candidates) {
            vector<vector<ll>> cur_a_part = a_part;
            for (auto stamp_id : stamp_ids) {
                rep(ii, 3) rep(jj, 3) {
                    cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                }
            }
            ll sum = 0;
            rep(ii, 3) rep(jj, 3) {
                sum += cur_a_part[ii][jj];
            }
            if (sum > best_mod_a_sum) {
                best_mod_a_sum = sum;
                best_stamp_ids = stamp_ids;
                best_a_part = cur_a_part;
            }
        }
        for (auto best_stamp_id : best_stamp_ids) {
            ev.apply_stamp(in.stamps[best_stamp_id], i_rev, j);
        }

        cerr << "left bottom: " << ev.sum << endl;

        if (ev.sum > final_score) {
            final_ans = best_ans;
            final_score = ev.sum;
            final_a = a;
            final_i = i_rev;
            final_j = j;
        }
    }

    if (enable_rt) {
        // 右上から順にスタンプを置いていく
        vector<Step> best_ans;
        vector<vector<ll>> a(in.n, vector<ll>(in.n));
        rep(i, in.n) rep(j, in.n) a[i][j] = in.a[i][j];
        Evaluator ev(a);
        rep(i, in.n - 3) {
            rep(j, in.n - 3) {
                int j_rev = in.n - 3 - j;
                vector<vector<ll>> a_part(3, vector<ll>(3));
                rep(ii, 3) rep(jj, 3) {
                    a_part[ii][jj] = a[i + ii][j_rev + jj];
                }
                ll best_stamp_id = -1;
                ll best_mod_a = -1;
                vector<vector<ll>> best_a_part;
                rep(k, in.m) {
                    vector<vector<ll>> cur_a_part = a_part;
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[k][ii][jj]) % 998244353;
                    }
                    if (cur_a_part[0][2] > best_mod_a) {
                        best_mod_a = cur_a_part[0][2];
                        best_stamp_id = k;
                        best_a_part = cur_a_part;
                    }
                }
                best_ans.push_back({best_stamp_id, i, j_rev});
                ev.apply_stamp(in.stamps[best_stamp_id], i, j_rev);
                rep(ii, 3) rep(jj, 3) {
                    a[i + ii][j_rev + jj] = best_a_part[ii][jj];
                }
            }
        }

        // 次は左端の列
        rep(i, in.n - 3) {
            int j = in.n - 3;
            int j_rev = in.n - 3 - j;
            vector<vector<ll>> a_part(3, vector<ll>(3));
            rep(ii, 3) rep(jj, 3) {
                a_part[ii][jj] = a[i + ii][j_rev + jj];
            }
            vector<ll> best_stamp_ids;
            ll best_mod_a_sum = -1;
            vector<vector<ll>> best_a_part;
            vector<vector<ll>> stamp_ids_candidates;
            stamp_ids_candidates = generate_combinations(in.m, 3);
            for (auto stamp_ids : stamp_ids_candidates) {
                vector<vector<ll>> cur_a_part = a_part;
                for (auto stamp_id : stamp_ids) {
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                    }
                }
                ll sum = 0;
                rep(jj, 3) {
                    sum += cur_a_part[0][jj];
                }
                if (sum > best_mod_a_sum) {
                    best_mod_a_sum = sum;
                    best_stamp_ids = stamp_ids;
                    best_a_part = cur_a_part;
                }
            }
            for (auto best_stamp_id : best_stamp_ids) {
                best_ans.push_back({best_stamp_id, i, j_rev});
                ev.apply_stamp(in.stamps[best_stamp_id], i, j_rev);
            }
            rep(ii, 3) rep(jj, 3) {
                a[i + ii][j_rev + jj] = best_a_part[ii][jj];
            }
        }

        // 次は下端の行
        rep(j, in.n - 3) {
            int j_rev = in.n - 3 - j;
            int i = in.n - 3;
            vector<vector<ll>> a_part(3, vector<ll>(3));
            rep(ii, 3) rep(jj, 3) {
                a_part[ii][jj] = a[i + ii][j_rev + jj];
            }
            vector<ll> best_stamp_ids;
            ll best_mod_a_sum = -1;
            vector<vector<ll>> best_a_part;
            vector<vector<ll>> stamp_ids_candidates;
            stamp_ids_candidates = generate_combinations(in.m, 3);
            for (auto stamp_ids : stamp_ids_candidates) {
                vector<vector<ll>> cur_a_part = a_part;
                for (auto stamp_id : stamp_ids) {
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                    }
                }
                ll sum = 0;
                rep(ii, 3) {
                    sum += cur_a_part[ii][2];
                }
                if (sum > best_mod_a_sum) {
                    best_mod_a_sum = sum;
                    best_stamp_ids = stamp_ids;
                    best_a_part = cur_a_part;
                }
            }
            for (auto best_stamp_id : best_stamp_ids) {
                best_ans.push_back({best_stamp_id, i, j_rev});
                ev.apply_stamp(in.stamps[best_stamp_id], i, j_rev);
            }
            rep(ii, 3) rep(jj, 3) {
                a[i + ii][j_rev + jj] = best_a_part[ii][jj];
            }
        }

        int i = in.n - 3;
        int j = in.n - 3;
        int j_rev = in.n - 3 - j;
        vector<vector<ll>> a_part(3, vector<ll>(3));
        rep(ii, 3) rep(jj, 3) {
            a_part[ii][jj] = a[i + ii][j_rev + jj];
        }
        vector<ll> best_stamp_ids;
        ll best_mod_a_sum = -1;
        vector<vector<ll>> best_a_part;
        vector<vector<ll>> stamp_ids_candidates;
        stamp_ids_candidates = generate_combinations(in.m, 4);
        for (auto stamp_ids : stamp_ids_candidates) {
            vector<vector<ll>> cur_a_part = a_part;
            for (auto stamp_id : stamp_ids) {
                rep(ii, 3) rep(jj, 3) {
                    cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                }
            }
            ll sum = 0;
            rep(ii, 3) rep(jj, 3) {
                sum += cur_a_part[ii][jj];
            }
            if (sum > best_mod_a_sum) {
                best_mod_a_sum = sum;
                best_stamp_ids = stamp_ids;
                best_a_part = cur_a_part;
            }
        }
        for (auto best_stamp_id : best_stamp_ids) {
            ev.apply_stamp(in.stamps[best_stamp_id], i, j_rev);
        }

        cerr << "left bottom: " << ev.sum << endl;

        if (ev.sum > final_score) {
            final_ans = best_ans;
            final_score = ev.sum;
            final_a = a;
            final_i = i;
            final_j = j_rev;
        }
    }

    if (enable_rb) {
        // 右下から順にスタンプを置いていく
        vector<Step> best_ans;
        vector<vector<ll>> a(in.n, vector<ll>(in.n));
        rep(i, in.n) rep(j, in.n) a[i][j] = in.a[i][j];
        Evaluator ev(a);
        rep(i, in.n - 3) {
            int i_rev = in.n - 3 - i;
            rep(j, in.n - 3) {
                int j_rev = in.n - 3 - j;
                vector<vector<ll>> a_part(3, vector<ll>(3));
                rep(ii, 3) rep(jj, 3) {
                    a_part[ii][jj] = a[i_rev + ii][j_rev + jj];
                }
                ll best_stamp_id = -1;
                ll best_mod_a = -1;
                vector<vector<ll>> best_a_part;
                rep(k, in.m) {
                    vector<vector<ll>> cur_a_part = a_part;
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[k][ii][jj]) % 998244353;
                    }
                    if (cur_a_part[2][2] > best_mod_a) {
                        best_mod_a = cur_a_part[2][2];
                        best_stamp_id = k;
                        best_a_part = cur_a_part;
                    }
                }
                best_ans.push_back({best_stamp_id, i_rev, j_rev});
                ev.apply_stamp(in.stamps[best_stamp_id], i_rev, j_rev);
                rep(ii, 3) rep(jj, 3) {
                    a[i_rev + ii][j_rev + jj] = best_a_part[ii][jj];
                }
            }
        }

        // 次は左端の列
        rep(i, in.n - 3) {
            int j = in.n - 3;
            int j_rev = in.n - 3 - j;
            int i_rev = in.n - 3 - i;
            vector<vector<ll>> a_part(3, vector<ll>(3));
            rep(ii, 3) rep(jj, 3) {
                a_part[ii][jj] = a[i_rev + ii][j_rev + jj];
            }
            vector<ll> best_stamp_ids;
            ll best_mod_a_sum = -1;
            vector<vector<ll>> best_a_part;
            vector<vector<ll>> stamp_ids_candidates;
            stamp_ids_candidates = generate_combinations(in.m, 3);
            for (auto stamp_ids : stamp_ids_candidates) {
                vector<vector<ll>> cur_a_part = a_part;
                for (auto stamp_id : stamp_ids) {
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                    }
                }
                ll sum = 0;
                rep(jj, 3) {
                    sum += cur_a_part[2][jj];
                }
                if (sum > best_mod_a_sum) {
                    best_mod_a_sum = sum;
                    best_stamp_ids = stamp_ids;
                    best_a_part = cur_a_part;
                }
            }
            for (auto best_stamp_id : best_stamp_ids) {
                best_ans.push_back({best_stamp_id, i_rev, j_rev});
                ev.apply_stamp(in.stamps[best_stamp_id], i_rev, j_rev);
            }
            rep(ii, 3) rep(jj, 3) {
                a[i_rev + ii][j_rev + jj] = best_a_part[ii][jj];
            }
        }

        // 次は上端の行
        rep(j, in.n - 3) {
            int j_rev = in.n - 3 - j;
            int i = in.n - 3;
            int i_rev = in.n - 3 - i;
            vector<vector<ll>> a_part(3, vector<ll>(3));
            rep(ii, 3) rep(jj, 3) {
                a_part[ii][jj] = a[i_rev + ii][j_rev + jj];
            }
            vector<ll> best_stamp_ids;
            ll best_mod_a_sum = -1;
            vector<vector<ll>> best_a_part;
            vector<vector<ll>> stamp_ids_candidates;
            stamp_ids_candidates = generate_combinations(in.m, 3);
            for (auto stamp_ids : stamp_ids_candidates) {
                vector<vector<ll>> cur_a_part = a_part;
                for (auto stamp_id : stamp_ids) {
                    rep(ii, 3) rep(jj, 3) {
                        cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                    }
                }
                ll sum = 0;
                rep(ii, 3) {
                    sum += cur_a_part[ii][2];
                }
                if (sum > best_mod_a_sum) {
                    best_mod_a_sum = sum;
                    best_stamp_ids = stamp_ids;
                    best_a_part = cur_a_part;
                }
            }
            for (auto best_stamp_id : best_stamp_ids) {
                best_ans.push_back({best_stamp_id, i_rev, j_rev});
                ev.apply_stamp(in.stamps[best_stamp_id], i_rev, j_rev);
            }
            rep(ii, 3) rep(jj, 3) {
                a[i_rev + ii][j_rev + jj] = best_a_part[ii][jj];
            }
        }

        int i = in.n - 3;
        int j = in.n - 3;
        int j_rev = in.n - 3 - j;
        int i_rev = in.n - 3 - i;
        vector<vector<ll>> a_part(3, vector<ll>(3));
        rep(ii, 3) rep(jj, 3) {
            a_part[ii][jj] = a[i_rev + ii][j_rev + jj];
        }
        vector<ll> best_stamp_ids;
        ll best_mod_a_sum = -1;
        vector<vector<ll>> best_a_part;
        vector<vector<ll>> stamp_ids_candidates;
        stamp_ids_candidates = generate_combinations(in.m, 4);
        for (auto stamp_ids : stamp_ids_candidates) {
            vector<vector<ll>> cur_a_part = a_part;
            for (auto stamp_id : stamp_ids) {
                rep(ii, 3) rep(jj, 3) {
                    cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
                }
            }
            ll sum = 0;
            rep(ii, 3) rep(jj, 3) {
                sum += cur_a_part[ii][jj];
            }
            if (sum > best_mod_a_sum) {
                best_mod_a_sum = sum;
                best_stamp_ids = stamp_ids;
                best_a_part = cur_a_part;
            }
        }
        for (auto best_stamp_id : best_stamp_ids) {
            ev.apply_stamp(in.stamps[best_stamp_id], i_rev, j_rev);
        }

        cerr << "right bottom: " << ev.sum << endl;

        if (ev.sum > final_score) {
            final_ans = best_ans;
            final_score = ev.sum;
            final_a = a;
            final_i = i_rev;
            final_j = j_rev;
        }
    }

    Evaluator ev(final_a);
    vector<vector<ll>> a_part(3, vector<ll>(3));
    rep(ii, 3) rep(jj, 3) {
        a_part[ii][jj] = final_a[final_i + ii][final_j + jj];
    }
    vector<ll> best_stamp_ids;
    ll best_mod_a_sum = -1;
    vector<vector<ll>> best_a_part;
    vector<vector<ll>> stamp_ids_candidates;
    stamp_ids_candidates = generate_combinations(in.m, 5);
    for (auto stamp_ids : stamp_ids_candidates) {
        vector<vector<ll>> cur_a_part = a_part;
        for (auto stamp_id : stamp_ids) {
            rep(ii, 3) rep(jj, 3) {
                cur_a_part[ii][jj] = (cur_a_part[ii][jj] + in.stamps[stamp_id][ii][jj]) % 998244353;
            }
        }
        ll sum = 0;
        rep(ii, 3) rep(jj, 3) {
            sum += cur_a_part[ii][jj];
        }
        if (sum > best_mod_a_sum) {
            best_mod_a_sum = sum;
            best_stamp_ids = stamp_ids;
            best_a_part = cur_a_part;
        }
    }
    for (auto best_stamp_id : best_stamp_ids) {
        final_ans.push_back({best_stamp_id, final_i, final_j});
        ev.apply_stamp(in.stamps[best_stamp_id], final_i, final_j);
    }

    cerr << "final: " << ev.sum << endl;

    println(final_ans.size());
    for (auto s : final_ans) {
        println(s.stamp_id, s.x, s.y);
    }

    return 0;
}
