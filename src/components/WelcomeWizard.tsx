import React, { useState } from 'react';

interface WelcomeWizardProps {
  onComplete: () => void;
  steps?: WelcomeStep[];
}

interface WelcomeStep {
  id: string;
  title: string;
  description: string;
  component: React.ReactNode;
}

export const WelcomeWizard: React.FC<WelcomeWizardProps> = ({ 
  onComplete,
  steps = defaultSteps 
}) => {
  const [currentStep, setCurrentStep] = useState(0);

  const handleNext = () => {
    if (currentStep < steps.length - 1) {
      setCurrentStep(currentStep + 1);
    } else {
      onComplete();
    }
  };

  const handleBack = () => {
    if (currentStep > 0) {
      setCurrentStep(currentStep - 1);
    }
  };

  const handleSkip = () => {
    onComplete();
  };

  const currentStepData = steps[currentStep];

  return (
    <div className="welcome-wizard">
      <div className="wizard-container">
        <div className="wizard-progress">
          {steps.map((step, index) => (
            <div
              key={step.id}
              className={`progress-step ${
                index === currentStep ? 'active' : 
                index < currentStep ? 'completed' : ''
              }`}
              onClick={() => index <= currentStep && setCurrentStep(index)}
            >
              <div className="step-number">{index + 1}</div>
              <div className="step-title">{step.title}</div>
            </div>
          ))}
        </div>

        <div className="wizard-content">
          <h2 className="step-title-large">{currentStepData.title}</h2>
          <p className="step-description">{currentStepData.description}</p>
          <div className="step-component">
            {currentStepData.component}
          </div>
        </div>

        <div className="wizard-navigation">
          {currentStep > 0 && (
            <button
              className="nav-button back-button"
              onClick={handleBack}
            >
              ← 返回
            </button>
          )}
          
          <button
            className="nav-button skip-button"
            onClick={handleSkip}
          >
            跳过设置
          </button>

          <button
            className="nav-button next-button"
            onClick={handleNext}
          >
            {currentStep === steps.length - 1 ? '完成' : '下一步 →'}
          </button>
        </div>
      </div>
    </div>
  );
};

const defaultSteps: WelcomeStep[] = [
  {
    id: 'welcome',
    title: '欢迎使用 MindCortex',
    description: '让我们一起配置您的 AI 知识管理系统',
    component: (
      <div className="welcome-content">
        <div className="welcome-icon">🧠</div>
        <h3>MindCortex - 您的 AI 知识库</h3>
        <p>
          MindCortex 帮助您从各种 AI 工具中提取知识，
          构建个人知识库，并通过智能搜索快速快速找到信息。
        </p>
      </div>
    ),
  },
  {
    id: 'complete',
    title: '配置完成',
    description: '您的 MindCortex 已准备就绪！',
    component: (
      <div className="complete-config">
        <div className="success-icon">✅</div>
        <h3>配置完成！</h3>
        <p>您的 MindCortex 已成功配置。</p>
      </div>
    ),
  },
];
