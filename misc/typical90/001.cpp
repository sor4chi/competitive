#include <iostream>
#include <vector>
using namespace std;

int main() {
  int n, l;
  cin >> n >> l;
  int k;
  cin >> k;
  vector<int> a(n);
  for (int i = 0; i < n; i++) {
    cin >> a[i];
  }
  // x 以上の長さで分割できるか
  // 分割できる -> 分割した最小の長さが x 以上
  auto can_split_with = [&](int x) {
    int cnt = 0;
    int prev = 0;
    for (int i = 0; i < n; i++) {
      if (a[i] - prev >= x && l - a[i] >= x) {
        cnt++;
        prev = a[i];
      }
    }
    return cnt >= k;
  };
  int left = 0;
  int right = l;
  while (right - left > 1) {
    int mid = (left + right) / 2;
    if (can_split_with(mid)) {
      left = mid;
    } else {
      right = mid;
    }
  }
  cout << left << endl;
  return 0;
}
