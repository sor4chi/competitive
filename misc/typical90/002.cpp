#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

bool is_parentheses_valid(const string &s) {
  int count = 0;
  for (char c : s) {
    if (c == '(') {
      count++;
    } else {
      count--;
    }
    if (count < 0) {
      return false;
    }
  }
  return count == 0;
}

int main() {
  int N;
  cin >> N;
  if (N % 2 == 1) {
    return 0;
  }
  vector<string> result;
  // 2^N回のbit全探索
  for (int i = 0; i < (1 << N); i++) {
    string s;
    for (int j = 0; j < N; j++) {
      if (i & (1 << j)) { // iのjビット目が立っているか
        s += '(';
      } else {
        s += ')';
      }
    }
    if (is_parentheses_valid(s)) {
      result.push_back(s);
    }
  }

  sort(result.begin(), result.end());
  for (const string &s : result) {
    cout << s << endl;
  }

  return 0;
}
