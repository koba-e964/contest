#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;
  
#define rep(i, n) for (int i = 0; i < int(n); ++i)
typedef long long i64;

const int N = 15;
int main(void) {
  int fb[N] = {2, 3, 5, 7,
 11, 13, 17, 19, 23,
 29, 31, 37, 41,
43, 47,};
  bool res[N];
  int cnt = 0;
  rep(i, N) {
    cout << fb[i] << endl;
    string r;
    cin >> r;
    res[i] = r == "yes";
    cnt += res[i] ? 1 : 0;
  }
  if (cnt >= 2) {cout <<"composite"<<endl;return 0;}
if (cnt==0){cout <<"prime"<<endl;return 0;}
int idx = -1;
rep(i,N) {if(res[i])idx=i;}
bool ans = false;
if (fb[idx] <= 10) {
 cout<<fb[idx]*fb[idx] <<endl;
 string s;
 cin >>s;
 ans = s == "yes";
}
cout << (ans ? "composite" : "prime") << endl;
}
