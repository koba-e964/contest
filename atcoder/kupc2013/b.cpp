#include <string>
#include <iostream>
#include <cmath>

using namespace std;
#define N 6
#define M 11
int n,x,m;
int l[M],r[M],s[M];

int maxv;
int maxp;
void check(int pat){
	int a[N];
	int oldpat=pat;
	for(int i=0;i<n;i++){
		a[i]=pat%(x+1);
		pat/=x+1;
	}
	for(int i=0;i<m;i++){
		int sum=0;
		for(int j=l[i];j<=r[i];j++){
			sum+=a[j];
		}
		if(sum!=s[i])return;
	}
	int sum=0;
	for(int i=0;i<n;i++)sum+=a[i];
	if(maxv<=sum){
		maxv=sum;
		maxp=oldpat;
	}
}

int main(void){
	cin>>n>>x>>m;
	for(int i=0;i<m;i++){
		cin>>l[i]>>r[i]>>s[i];
		l[i]--;
		r[i]--;
	}
	maxv=0;
	maxp=-1;
	int k=pow(x+1,n);
	for(int i=0;i<k;i++){
		check(i);
	}
	if(maxp==-1){
		cout<<"-1"<<endl;
		return 0;
	}
	for(int i=0;i<n;i++){
		cout<<maxp%(x+1)<<(i==n-1?"\n":" ");
		maxp/=x+1;
	}
}