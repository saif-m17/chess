import torch
import torch.nn as nn
import torch.nn.functional as F

class ResBlock(nn.Module):
    def __init__(self, channels):
        super().__init__()
        self.conv1 = nn.Conv2d(in_channels=channels, out_channels=channels, kernel_size=3, padding=1, bias=False)
        self.bn1 = nn.BatchNorm2d(num_features=channels)
        self.conv2 = nn.Conv2d(in_channels=channels, out_channels=channels, kernel_size=3, padding=1, bias=False)
        self.bn2 = nn.BatchNorm2d(num_features=channels)
    
    def forward(self, x):
        residual = x
        out = F.relu(self.bn1(self.conv1(x)))
        out = self.bn2(self.conv2(out))
        out = out + residual
        return F.relu(out)


class ChessCNN(nn.Module):
    def __init__(self, input_channels=20, num_actions=20480):
        super().__init__()
        self.input_conv = nn.Conv2d(in_channels=input_channels, out_channels=256, kernel_size=3, padding=1, bias=False)
        self.input_bn = nn.BatchNorm2d(num_features=256)

        self.residual_blocks = nn.ModuleList([ResBlock(256) for _ in range(10)])

        self.policy_conv = nn.Conv2d(in_channels=256, out_channels=32, kernel_size=1, bias=False)
        self.policy_bn = nn.BatchNorm2d(num_features=32)
        self.policy_fc = nn.Linear(in_features=32 * 8 * 8, out_features=num_actions)

        self.value_conv = nn.Conv2d(in_channels=256, out_channels=8, kernel_size=1, bias=False)
        self.value_bn = nn.BatchNorm2d(num_features=8)
        self.value_fc1 = nn.Linear(in_features=8 * 8 * 8, out_features=256)
        self.value_fc2 = nn.Linear(in_features=256, out_features=1)

    
    def forward(self, x):
        x = self.input_conv(F.relu(self.input_bn(x)))
        for block in self.residual_blocks:
            x = block(x)
        
        policy = F.relu(self.policy_bn(self.policy_conv(x)))
        policy = policy.view(policy.size(0), -1)
        policy = self.policy_fc(policy)

        value = F.relu(self.value_bn(self.value_conv(x)))
        value = value.view(value.size(0), -1)
        value = F.relu(self.value_fc1(value))
        value = torch.tanh(self.value_fc2(value))

        return policy, value

