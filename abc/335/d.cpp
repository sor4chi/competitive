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

const pair<int, int> directions[4] = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};

vector<vector<ll>> generate_uzumaki(int n) {
    vector<vector<ll>> board(n, vector<ll>(n, 0));
    int x = 0, y = 0;
    int direction = 0;
    int cnt = 1;
    while (cnt <= n * n) {
        board[y][x] = cnt;
        int dx = directions[direction].first;
        int dy = directions[direction].second;
        // もし外にはみ出すか、すでに数字が入っていたら
        if (x + dx == n || x + dx == -1 || y + dy == n || y + dy == -1 ||
            board[y + dy][x + dx] != 0) {
            // 右に曲がる
            direction = (direction + 1) % 4;
        }
        x += directions[direction].first;
        y += directions[direction].second;
        cnt++;
    }
    return board;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    int n;
    input(n);
    vector<vector<ll>> board = generate_uzumaki(n);

    rep(i, n) {
        rep(j, n) {
            if (i == n / 2 && j == n / 2) {
                printf("T ");
            } else {
                printf("%d ", board[i][j]);
            }
            if (j == n - 1) {
                printf("\n");
            }
        }
    }

    return 0;
}
